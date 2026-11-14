//! `nsx agent plan`, end to end
//!
//! The real binary plans a generated ticket against the console the
//! environment names, and the console's record is read back.

use crate::e2e::{generated::generated, played::played, recorded::recorded};

const NEEDS: &str = "The end-to-end run needs MCP_CONSOLE_URL and LANGDRIFT_SECRET: no console is a default.";

fn connection() -> (String, String) {
	let console = std::env::var("MCP_CONSOLE_URL").expect(NEEDS);
	let secret = std::env::var("LANGDRIFT_SECRET").expect(NEEDS);

	(console, secret)
}

fn lines(printed: &[u8]) -> Vec<String> {
	String::from_utf8_lossy(printed)
		.lines()
		.map(String::from)
		.collect()
}

#[tokio::test]
#[ignore = "needs a console whose team saved its Anthropic key and offer"]
async fn a_generated_ticket_is_planned_to_its_handoff_and_the_console_holds_the_plan()
 {
	let (console, secret) = connection();
	let ticket = generated();

	let played = played(&ticket, &console, &secret);

	let printed = lines(&played.plan.stdout);
	let record = recorded(played.project.path(), &console, &ticket.slug).await;

	assert!(played.plan.status.success(), "{:?}", played.plan);
	assert_eq!(
		printed.last().map(String::as_str),
		Some(
			"# next: once the team authorizes, nsx agent run applies the recorded intent"
		)
	);
	assert!(printed[1].ends_with(&format!("read {}", ticket.id())));
	assert_eq!(record["status"], "plan");
	assert_eq!(record["proposed"], true);
	assert_eq!(record["ticket"]["number"], ticket.number.as_str());
	assert_eq!(record["ticket"]["slug"], ticket.slug.as_str());
}

#[tokio::test]
#[ignore = "needs a console whose team saved no Anthropic key"]
async fn without_the_teams_key_a_generated_ticket_stops_as_the_simulation_shows()
 {
	let (console, secret) = connection();
	let ticket = generated();

	let played = played(&ticket, &console, &secret);

	let printed = lines(&played.plan.stdout);
	let record = recorded(played.project.path(), &console, &ticket.slug).await;
	let runtime = played
		.project
		.path()
		.join(".langdrift/microruntimes/runtimes")
		.join(&ticket.slug);

	assert_eq!(played.plan.status.code(), Some(1));
	assert_eq!(
		printed[printed.len() - 2..],
		[
			"✗ judge      no Anthropic key · add one on the console's Settings → AI Models",
			"# plan stopped · no draft written · nothing was paid for",
		]
	);
	assert!(!runtime.join("runtime.md").exists());
	assert_eq!(record["status"], "error");
	assert_eq!(record["judgeUnavailable"], "model-key-missing");
	assert_eq!(record["ticket"]["slug"], ticket.slug.as_str());
}
