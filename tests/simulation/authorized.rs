//! A planned project whose intent the team authorized, with the choice a
//! test names
//!
//! The project is set up as a team's is: `nsx init` has laid `.langdrift/`
//! down and `nsx agent plan` has proposed the intent. The test then stands
//! in for the team: it records the choice and authorizes it in `intent.json`,
//! and says in `harness/driver.json` how the coding agent is started.

use std::path::PathBuf;

use nsx::command::{init::Init, plan::Plan};
use nsx::contract::{decode::decode, encode::encode, intent::Intent};
use tempfile::TempDir;

use crate::simulation::{
	fake::{judge::judge, recorder::Recorder, session::session},
	indexed::indexed,
	project::project,
};

pub const INTENT: &str =
	".langdrift/microruntimes/runtimes/book-a-demo-runtime/intent.json";

pub const DRIVER: &str = ".langdrift/harness/driver.json";

// How a team would start its coding agent: the run's choice in the
// arguments, and where the agent's output holds what a call used.
const CONFIGURED: &str = r#"{
	"executable": "coding-agent",
	"args": ["--model", "{{model}}", "--effort", "{{effort}}", "{{prompt}}"],
	"passEnv": [],
	"usage": {
		"inputTokens": "usage.input_tokens",
		"outputTokens": "usage.output_tokens"
	}
}
"#;

pub async fn authorized(
	model: &str,
	effort: &str,
	checked: &[&str],
) -> TempDir {
	let project = project();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let judge = judge();
	let file = project.path().join(INTENT);

	Init.execute(&mut session).await.unwrap();
	indexed(project.path());

	session.judge = &judge;

	plan.execute(&mut session).await.unwrap();

	let mut intent =
		decode::<Intent>(&std::fs::read_to_string(&file).unwrap()).unwrap();

	intent.model = String::from(model);
	intent.effort = String::from(effort);
	intent.human.spec = checked.contains(&"spec");
	intent.human.tickets = checked.contains(&"tickets");
	intent.human.implement = checked.contains(&"implement");
	intent.human.review = checked.contains(&"review");
	intent.status = String::from("authorized");

	std::fs::write(&file, encode(&intent).unwrap()).unwrap();
	std::fs::write(project.path().join(DRIVER), CONFIGURED).unwrap();

	project
}
