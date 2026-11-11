//! The wire: what the console and its judge are sent, and what is read back

use std::path::PathBuf;

use nsx::console::{
	client::Client as Console, connection::Connection, error::Error,
	remote::Remote,
};
use nsx::contract::{report::Report, status::Status};
use nsx::harness::{criterion::Criterion, guardrail::Guardrail};
use nsx::judgment::{
	client::Client as Asker, judge::Judge, question::Question,
};

use crate::simulation::fake::served::served;

const SECRET: &str =
	"dft_1111111111111111111111111111111111111111111111111111111111111111";

#[tokio::test]
async fn a_report_is_one_tool_call_with_the_bearer_and_the_protocol_version() {
	let answer = r#"{"jsonrpc":"2.0","id":1,"result":{"structuredContent":{"runtimeId":"019a0000-0000-7000-8000-000000000001"},"content":[]}}"#;
	let (origin, received) = served("200 OK", answer).await;
	let remote = Remote::open(Connection {
		origin,
		secret: String::from(SECRET),
	})
	.unwrap();
	let report = Report {
		runtime_id: None,
		status: Status::Ready,
		plan: None,
		intent: None,
		run: None,
	};

	let reply = remote.report(&report).await.unwrap();
	let request = received.await.unwrap();
	let lowered = request.to_lowercase();
	let (_head, body) = request.split_once("\r\n\r\n").unwrap();
	let call: serde_json::Value = serde_json::from_str(body).unwrap();

	assert_eq!(reply.runtime_id, "019a0000-0000-7000-8000-000000000001");
	assert!(request.starts_with("POST /api/mcp HTTP/1.1"));
	assert!(lowered.contains("mcp-protocol-version: 2025-11-25"));
	assert!(lowered.contains("content-type: application/json"));
	assert!(request.contains(&format!("Bearer {SECRET}")));
	assert_eq!(call["method"], "tools/call");
	assert_eq!(call["params"]["name"], "langdrift.runtime.report");
	assert_eq!(
		call["params"]["arguments"],
		serde_json::json!({
			"status": "ready",
		})
	);
}

#[tokio::test]
async fn a_console_that_refuses_is_unavailable_and_the_secret_stays_unsaid() {
	let answer = r#"{"error":{"code":"unauthorized"}}"#;
	let (origin, _received) = served("401 Unauthorized", answer).await;
	let remote = Remote::open(Connection {
		origin,
		secret: String::from(SECRET),
	})
	.unwrap();
	let report = Report {
		runtime_id: None,
		status: Status::Ready,
		plan: None,
		intent: None,
		run: None,
	};

	let refusal = remote.report(&report).await.unwrap_err();

	assert!(
		matches!(refusal, Error::Unavailable { .. }),
		"was {refusal:?}"
	);
	assert!(
		refusal.to_string().contains("unauthorized"),
		"said {refusal}"
	);
	assert!(!refusal.to_string().contains(SECRET));
}

#[tokio::test]
async fn a_question_is_posted_to_the_judge_and_its_text_comes_back() {
	let answer = r#"{"text":"{\"criteria\":[]}","refused":false,"reason":"","judgeVersion":"console-judge-1"}"#;
	let (origin, received) = served("200 OK", answer).await;
	let asker = Asker::open(Connection {
		origin,
		secret: String::from(SECRET),
	})
	.unwrap();
	let guardrail = Guardrail {
		path: PathBuf::from(".langdrift/harness/guardrails/0001-finops.gr.md"),
		name: String::from("0001-finops.gr.md"),
		short: String::from("0001-finops"),
		criteria: vec![Criterion {
			id: String::from("FIN-02"),
			rule: String::from("Estimate before any paid call."),
			plan: true,
			run: true,
		}],
	};
	let question = Question::Criteria {
		guardrail: &guardrail,
	};

	let text = asker.ask(&question).await.unwrap();
	let request = received.await.unwrap();
	let (_head, body) = request.split_once("\r\n\r\n").unwrap();
	let asked: serde_json::Value = serde_json::from_str(body).unwrap();

	assert_eq!(text, r#"{"criteria":[]}"#);
	assert!(request.starts_with("POST /api/judge HTTP/1.1"));
	assert!(request.contains(&format!("Bearer {SECRET}")));
	assert_eq!(asked["kind"], "criteria");
	assert_eq!(asked["model"], "claude-sonnet-5-5");
	assert_eq!(asked["maxTokens"], 2000);
	assert_eq!(asked["promptVersion"], "nsx-1");
	assert!(
		asked["prompt"]
			.as_str()
			.unwrap()
			.contains("- FIN-02: Estimate before any paid call.")
	);
	assert!(asked["system"].as_str().unwrap().contains("\"statement\""));
	assert_eq!(asked.as_object().unwrap().len(), 6);
}

#[tokio::test]
async fn a_judge_that_is_not_configured_stops_with_the_consoles_reason() {
	let answer = r#"{"code":"UNAVAILABLE","reason":"judge-not-configured"}"#;
	let (origin, _received) = served("503 Service Unavailable", answer).await;
	let asker = Asker::open(Connection {
		origin,
		secret: String::from(SECRET),
	})
	.unwrap();
	let guardrail = Guardrail {
		path: PathBuf::from("guardrail.gr.md"),
		name: String::from("guardrail.gr.md"),
		short: String::from("guardrail"),
		criteria: Vec::new(),
	};
	let question = Question::Criteria {
		guardrail: &guardrail,
	};

	let refusal = asker.ask(&question).await.unwrap_err();

	assert_eq!(
		refusal.to_string(),
		"The console's judge did not answer: the console answered 503 Service Unavailable judge-not-configured."
	);
}
