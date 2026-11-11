//! The reports a connected plan sends: every record as it is written

use std::path::PathBuf;

use nsx::command::{init::Init, plan::Plan};
use nsx::console::connection::Connection;
use nsx::contract::{cards::Cards, schema::schema, status::Status};
use nsx::files::{layout::Layout, root::Root};

use crate::simulation::{
	diff::diff,
	fake::{
		console::{Console, RUNTIME},
		judge::judge,
		recorder::Recorder,
		session::session,
	},
	golden::golden,
	indexed::indexed,
	pending,
	printed::printed,
	project::project,
};

const SECRET: &str =
	"dft_2222222222222222222222222222222222222222222222222222222222222222";

const KEPT: &str = ".langdrift/agent/state/console/book-a-demo-runtime.id";

#[tokio::test]
async fn a_connected_plan_reports_each_write_and_prints_the_same_lines() {
	let project = project();
	let recorder = Recorder::default();
	let console = Console::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let judge = judge();

	Init.execute(&mut session).await.unwrap();
	indexed(project.path());

	session.judge = &judge;
	session.console = &console;

	plan.execute(&mut session).await.unwrap();

	let reports = console.reports.borrow();
	let first = reports.first().unwrap();
	let last = reports.last().unwrap();
	let differences = diff(&golden("plan"), &printed(&recorder), &pending::PLAN);
	let cards = schema::<Cards>();
	let valid = reports.iter().all(|report| {
		jsonschema::is_valid(&cards, &serde_json::to_value(&report.plan).unwrap())
	});

	assert!(differences.is_empty(), "{}", differences.join("\n"));
	assert!(reports.len() > 20, "sent {}", reports.len());
	assert!(valid);
	assert_eq!(
		(first.runtime_id.as_deref(), first.status),
		(None, Status::Analysis)
	);
	assert!(first.plan.as_ref().unwrap().workflow.is_some());
	assert!(first.intent.is_none());
	assert_eq!(reports[1].runtime_id.as_deref(), Some(RUNTIME));
	assert_eq!(last.status, Status::Plan);
	assert_eq!(last.intent.as_ref().unwrap().status, "proposed");
	assert!(
		last
			.plan
			.as_ref()
			.unwrap()
			.plan
			.as_ref()
			.unwrap()
			.evidence
			.is_some()
	);
	assert_eq!(
		std::fs::read_to_string(project.path().join(KEPT)).unwrap(),
		RUNTIME
	);
}

#[tokio::test]
async fn a_second_plan_reports_into_the_runtime_the_console_named() {
	let project = project();
	let recorder = Recorder::default();
	let console = Console::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let judge = judge();

	Init.execute(&mut session).await.unwrap();
	indexed(project.path());

	session.judge = &judge;
	session.console = &console;

	plan.execute(&mut session).await.unwrap();

	let sent = console.reports.borrow().len();

	plan.execute(&mut session).await.unwrap();

	let reports = console.reports.borrow();

	assert_eq!(reports[sent].runtime_id.as_deref(), Some(RUNTIME));
}

#[tokio::test]
async fn a_console_that_is_down_is_asked_once_and_stops_nothing() {
	let project = project();
	let recorder = Recorder::default();
	let console = Console {
		down: true,
		..Console::default()
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let judge = judge();

	Init.execute(&mut session).await.unwrap();
	indexed(project.path());

	session.judge = &judge;
	session.console = &console;

	plan.execute(&mut session).await.unwrap();

	let differences = diff(&golden("plan"), &printed(&recorder), &pending::PLAN);

	assert!(differences.is_empty(), "{}", differences.join("\n"));
	assert_eq!(console.reports.borrow().len(), 1);
	assert!(!project.path().join(KEPT).exists());
}

#[tokio::test]
async fn init_with_a_secret_records_the_connection_and_sends_nothing() {
	let project = tempfile::tempdir().unwrap();
	let recorder = Recorder::default();
	let console = Console::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let offered = |name: &str| match name {
		"LANGDRIFT_SECRET" => Some(String::from(SECRET)),
		"MCP_CONSOLE_URL" => Some(String::from("http://localhost:3001")),
		_ => None,
	};

	session.console = &console;
	session.environment = &offered;

	Init.execute(&mut session).await.unwrap();

	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let recorded = Connection::load(&root, &Layout::under(&root))
		.unwrap()
		.unwrap();

	assert!(recorder.lines.borrow().is_empty());
	assert!(console.reports.borrow().is_empty());
	assert_eq!(recorded.origin, "http://localhost:3001");
	assert_eq!(recorded.secret, SECRET);
}

#[tokio::test]
async fn init_with_an_address_nsx_will_not_use_stops_and_never_says_the_secret()
{
	let project = tempfile::tempdir().unwrap();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let offered = |name: &str| match name {
		"LANGDRIFT_SECRET" => Some(String::from(SECRET)),
		"MCP_CONSOLE_URL" => Some(String::from("http://console.example")),
		_ => None,
	};

	session.environment = &offered;

	let stop = Init.execute(&mut session).await.unwrap_err();

	assert!(stop.sentence.contains("the console's address"));
	assert!(!stop.sentence.contains(SECRET));
}

#[tokio::test]
async fn a_console_that_forgot_the_runtime_is_asked_again_with_no_id() {
	let project = project();
	let recorder = Recorder::default();
	let console = Console {
		forgets: Some(String::from("019a0000-0000-7000-8000-00000000dead")),
		..Console::default()
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let judge = judge();
	let kept = project.path().join(KEPT);

	Init.execute(&mut session).await.unwrap();
	indexed(project.path());
	std::fs::create_dir_all(kept.parent().unwrap()).unwrap();
	std::fs::write(&kept, "019a0000-0000-7000-8000-00000000dead").unwrap();

	session.judge = &judge;
	session.console = &console;

	plan.execute(&mut session).await.unwrap();

	let reports = console.reports.borrow();

	assert!(reports[0].runtime_id.is_some());
	assert_eq!(reports[1].runtime_id, None);
	assert_eq!(reports[2].runtime_id.as_deref(), Some(RUNTIME));
	assert!(reports.len() > 20, "sent {}", reports.len());
	assert_eq!(std::fs::read_to_string(&kept).unwrap(), RUNTIME);
}
