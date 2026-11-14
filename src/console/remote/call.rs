//! Calling one of the console's tools
//!
//! One function: it takes a tool's name and its arguments, calls the tool
//! once and gives back the tool's structured result, read strictly as its
//! answer.
//!
//! ## Reads and writes
//!
//! - posts one JSON-RPC `tools/call` to `<origin>/api/mcp`, with the
//!   protocol version `2025-11-25` and the secret as the bearer
//!
//! ## Rules
//!
//! - The console answers each request on its own, so no `initialize` is
//!   sent first.
//! - A console that answers `unknown-runtime` no longer knows the id sent.
//! - Anything else but a status of 200 with the tool's structured result is
//!   "unavailable": a refusal, a timeout, another status or another body.
//!   So is a result that does not read as the tool's answer. The reason is
//!   kept without the request's address.

use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

use crate::console::{
	error::Error,
	remote::{PATIENCE, PROTOCOL, Remote},
};

impl Remote {
	pub async fn call<Arguments: Serialize, Answer: DeserializeOwned>(
		&self,
		tool: &str,
		arguments: &Arguments,
	) -> Result<Answer, Error> {
		let unavailable = |cause: String| Error::Unavailable {
			cause,
		};
		let failed =
			|cause: reqwest::Error| unavailable(cause.without_url().to_string());
		let call = json!({
			"jsonrpc": "2.0",
			"id": 1,
			"method": "tools/call",
			"params": {
				"name": tool,
				"arguments": arguments,
			},
		});

		let answered = self
			.http
			.post(format!("{}/api/mcp", self.connection.origin))
			.timeout(PATIENCE)
			.header("mcp-protocol-version", PROTOCOL)
			.bearer_auth(&self.connection.secret)
			.json(&call)
			.send()
			.await
			.map_err(failed)?;

		let status = answered.status();

		let body = answered.json::<Value>().await.map_err(failed)?;

		let refusal = body
			.pointer("/error/message")
			.or_else(|| body.pointer("/error/code"));

		if refusal.and_then(Value::as_str) == Some("unknown-runtime") {
			return Err(Error::Unknown);
		}

		if let Some(refusal) = refusal {
			return Err(unavailable(format!("the console answered {refusal}")));
		}

		let result = body
			.pointer("/result/structuredContent")
			.filter(|_result| status.is_success())
			.cloned()
			.ok_or_else(|| unavailable(format!("the console answered {status}")))?;

		serde_json::from_value(result)
			.map_err(|cause| unavailable(cause.to_string()))
	}
}
