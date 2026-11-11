//! `nsx agent run`, past the gate: what a run leaves, and where it stops

use std::cell::RefCell;
use std::path::PathBuf;

use nsx::checks::{check::Check, standing::Standing};
use nsx::command::run::Run;
use nsx::contract::{decode::decode, outcome::Outcome, run::Run as Ran};
use nsx::evidence::usage::Usage;
use nsx::files::{layout::Layout, root::Root};
use nsx::local::execution::Execution;
use nsx::money::ledger::Ledger;
use rstest::rstest;

use crate::command::played::played;
use crate::simulation::{
	authorized::{DRIVER, authorized},
	fake::{
		answers::Answers,
		coder::{CODER, Coder},
		forged::{CHECKS, COMMIT, FORGED, Forged, TITLE},
		hasty::Hasty,
		host::{HOST, Host},
		hub::Hub,
		machine::Machine,
		moment::MOMENT,
		recorder::Recorder,
		session::session,
		spender::Spender,
	},
};

const FOLDER: &str = ".langdrift/microruntimes/runtimes/book-a-demo-runtime";

const LEDGER: &str =
	".langdrift/finops/ledger/book-a-demo-runtime-0001.ledger.json";

const EXECUTION: &str =
	".langdrift/agent/state/runs/book-a-demo-runtime-0001/execution.json";

#[tokio::test]
async fn a_run_that_passes_its_stages_leaves_each_of_them_in_its_records() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);

	let (ended, printed) = played(project.path(), CODER, 2).await;
	let record =
		decode::<Ran>(&std::fs::read_to_string(folder.join("run.json")).unwrap())
			.unwrap();
	let usage = decode::<Vec<Usage>>(
		&std::fs::read_to_string(folder.join("usage.json")).unwrap(),
	)
	.unwrap();
	let ledger = decode::<Ledger>(
		&std::fs::read_to_string(project.path().join(LEDGER)).unwrap(),
	)
	.unwrap();
	let stages = record.stages.unwrap();
	let states = stages
		.iter()
		.map(|stage| (stage.id.as_str(), stage.state.as_str()))
		.collect::<Vec<_>>();
	let effort = record.effort.unwrap();
	let calls = usage[0]
		.calls
		.iter()
		.map(|used| (used.stage.as_str(), used.skill.as_str()))
		.collect::<Vec<_>>();

	ended.unwrap();

	assert_eq!(printed.last().map(String::as_str), Some("next"));
	assert_eq!(record.outcome, Some(Outcome::Completed));
	assert_eq!(
		states,
		[
			("planning", "done"),
			("spec", "done"),
			("tickets", "done"),
			("implement", "done"),
			("dev-ticket", "done"),
			("review", "done"),
			("merge", "pending"),
		]
	);
	assert_eq!(stages[0].confirmed_by.as_deref(), Some("owner"));
	assert_eq!(stages[1].confirmed_by, None);
	assert!(stages.iter().take(4).all(|stage| stage.report.is_some()));
	assert_eq!(stages[4].closed_by.as_deref(), Some("person"));
	assert_eq!(stages[4].paid_calls, Some(0));
	assert!(stages[4].report.is_none());
	assert_eq!(
		(
			effort.applied_model.as_str(),
			effort.applied_effort.as_str()
		),
		("claude-sonnet-5-5", "medium")
	);
	assert_eq!(effort.suggested_model.as_deref(), Some("claude-sonnet-5-5"));
	assert_eq!(effort.recorded_at, MOMENT);
	assert_eq!(
		calls,
		[
			("planning", "grilling"),
			("spec", "to-spec"),
			("tickets", "to-tickets"),
			("implement", "implement · tdd"),
			("review", "code-review"),
		]
	);
	assert_eq!(
		(usage[0].input_tokens, usage[0].output_tokens),
		(Some(3_700_000), Some(240_000))
	);
	assert_eq!(
		(ledger.observed_input_tokens, ledger.observed_output_tokens),
		(Some(3_700_000), Some(240_000))
	);
	assert_eq!(ledger.observed_cents, Some(980));
	assert_eq!(ledger.released_cents, Some(160));
}

#[tokio::test]
async fn a_run_that_delivers_leaves_its_delivery_in_its_records() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);
	let episodes = project.path().join(".langdrift/microruntimes/episodes");
	let before = std::fs::read_dir(&episodes).unwrap().count();

	let (ended, printed) = played(project.path(), CODER, 2).await;
	let record =
		decode::<Ran>(&std::fs::read_to_string(folder.join("run.json")).unwrap())
			.unwrap();
	let execution = std::fs::read_to_string(project.path().join(EXECUTION));
	let runtime = record.runtime.unwrap();
	let delivery = record.delivery.unwrap();
	let pull = delivery.pull_request.unwrap();
	let checks = pull.checks.unwrap();
	let seat = record.safe_seat.unwrap();
	let after = std::fs::read_dir(&episodes).unwrap().count();
	let delivered = printed
		.iter()
		.skip_while(|id| *id != "codeReview")
		.map(String::as_str)
		.collect::<Vec<_>>();

	ended.unwrap();

	assert_eq!(
		delivered,
		[
			"codeReview",
			"runtimeReady",
			"completed",
			"pushed",
			"pullRequest",
			"checks",
			"checksPassed",
			"safeSeatMet",
			"runtimeStopped",
			"ready",
			"next",
		]
	);
	assert_eq!(
		(runtime.state.as_str(), runtime.deployments, runtime.removed),
		("stopped", 5, Some(5))
	);
	assert_eq!((runtime.cpu_millis, runtime.memory_mib), (1450, 2304));
	assert_eq!(runtime.left, Some(0));
	assert_eq!(
		delivery.branch.as_deref(),
		Some("feat/0031-book-a-demo-runtime")
	);
	assert_eq!(delivery.commit.as_deref(), Some("3f9c2e1"));
	assert_eq!((pull.number, pull.state.as_str()), (19, "ready for review"));
	assert_eq!(
		(checks.state.as_str(), checks.passed, checks.total),
		("passed", Some(4), Some(4))
	);
	assert_eq!(seat.safe_seat, "met");
	assert_eq!(seat.decided_at, MOMENT);
	assert!(execution.unwrap().contains("\"settled\": true"));
	assert_eq!(after, before + 1);
	assert_eq!(
		std::fs::read_to_string(
			project.path().join(".langdrift/agent/state/.gitignore")
		)
		.unwrap(),
		"*\n"
	);
}

#[rstest]
#[case::a_deployment_never_available(
	"deployment",
	"platform-redis",
	"codeReview",
	"kubectl rollout ended with the exit status 1."
)]
#[case::a_refused_push(
	"operation",
	"push",
	"completed",
	"git push ended with the exit status 1."
)]
#[case::a_pull_request_that_cannot_open(
	"operation",
	"open",
	"pushed",
	"gh pr ended with the exit status 1."
)]
#[case::a_check_that_fails(
	"check",
	"test",
	"checks",
	"The check test did not pass"
)]
#[case::checks_never_listed(
	"listing",
	"",
	"pullRequest",
	"listed no check after 20 answers"
)]
#[case::a_pull_request_that_cannot_be_marked_ready(
	"operation",
	"ready",
	"runtimeStopped",
	"gh pr ended with the exit status 1."
)]
#[tokio::test]
async fn a_delivery_that_stops_takes_the_runtime_down_and_leaves_a_draft(
	#[case] fault: &str,
	#[case] named: &'static str,
	#[case] last: &str,
	#[case] said: &str,
) {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);
	let episodes = project.path().join(".langdrift/microruntimes/episodes");
	let before = std::fs::read_dir(&episodes).unwrap().count();
	let recorder = Recorder::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let machine = Machine {
		failing: (fault == "deployment").then_some(named),
		..Machine::default()
	};
	let forged = Forged {
		refusing: (fault == "operation").then_some(named),
		failing: (fault == "check").then_some(named),
		unlisted: fault == "listing",
		..FORGED
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.answers = &answers;
	session.cluster = &machine;
	session.forge = &forged;

	let stop = Run.execute(&mut session).await.unwrap_err();
	let printed = recorder.lines.borrow();
	let record =
		decode::<Ran>(&std::fs::read_to_string(folder.join("run.json")).unwrap())
			.unwrap();
	let ledger = decode::<Ledger>(
		&std::fs::read_to_string(project.path().join(LEDGER)).unwrap(),
	)
	.unwrap();
	let execution =
		std::fs::read_to_string(project.path().join(EXECUTION)).unwrap();
	let state = record
		.delivery
		.and_then(|delivery| delivery.pull_request)
		.map(|pull| pull.state);
	let hosted = record.runtime.map(|runtime| (runtime.state, runtime.left));
	let after = std::fs::read_dir(&episodes).unwrap().count();

	assert!(stop.sentence.contains(said), "said {stop:?}");
	assert_eq!(printed.last().map(|line| line.id.as_str()), Some(last));
	assert_eq!(record.outcome, Some(Outcome::Stopped));
	assert!(execution.contains("\"settled\": true"), "{execution}");
	assert!(state.is_none_or(|state| state == "draft"));
	assert!(
		hosted.is_none_or(|hosted| hosted == (String::from("stopped"), Some(0)))
	);
	assert!(machine.spaces.borrow().is_empty());
	assert_eq!(ledger.released_cents, None);
	assert_eq!(after, before);
}

#[tokio::test]
async fn a_space_that_clings_stops_the_run_before_the_stopped_line() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let recorder = Recorder::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let clinging = Host {
		clinging: true,
		..HOST
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.answers = &answers;
	session.cluster = &clinging;

	let stop = Run.execute(&mut session).await.unwrap_err();
	let printed = recorder.lines.borrow();
	let execution =
		std::fs::read_to_string(project.path().join(EXECUTION)).unwrap();

	assert_eq!(
		stop.sentence.matches("is still in the cluster").count(),
		1,
		"said {stop:?}"
	);
	assert_eq!(
		printed.last().map(|line| line.id.as_str()),
		Some("safeSeatMet")
	);
	assert!(execution.contains("\"settled\": false"), "{execution}");
}

#[tokio::test]
async fn what_a_stopped_run_left_is_removed_before_anything_is_paid() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);
	let recorder = Recorder::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let machine = Machine::default();
	let failing = Coder {
		failing: Some("planning"),
		..CODER
	};
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let mut left = Execution::open(
		&root,
		&layout,
		"book-a-demo-runtime-0007",
		"book-a-demo-runtime",
	)
	.unwrap();
	let old = left.space.clone();
	let mut session = session(PathBuf::from(project.path()), &recorder);

	left.up(&root, &layout, &machine, &[]).await.unwrap();
	machine.asked.borrow_mut().clear();

	session.answers = &answers;
	session.agent = &failing;
	session.cluster = &machine;

	// The run stops at its first paid call: what was left is gone by then.
	Run.execute(&mut session).await.unwrap_err();

	let usage = decode::<Vec<Usage>>(
		&std::fs::read_to_string(folder.join("usage.json")).unwrap(),
	)
	.unwrap();

	assert_eq!(usage[0].calls.len(), 1);
	assert_eq!(
		*machine.asked.borrow(),
		[format!("close {old}"), format!("absent {old}")]
	);
	assert!(machine.spaces.borrow().is_empty());
	assert!(
		Execution::left(&root, &layout, "book-a-demo-runtime")
			.unwrap()
			.is_empty()
	);
}

#[tokio::test]
async fn a_run_that_stops_before_its_bring_up_asks_nothing_of_the_cluster() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let recorder = Recorder::default();
	let machine = Machine::default();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let mut session = session(PathBuf::from(project.path()), &recorder);

	Execution::open(
		&root,
		&layout,
		"book-a-demo-runtime-0007",
		"book-a-demo-runtime",
	)
	.unwrap();
	std::fs::remove_file(project.path().join(DRIVER)).unwrap();

	session.cluster = &machine;

	let stop = Run.execute(&mut session).await.unwrap_err();

	assert!(
		stop.sentence.contains("no driver configuration"),
		"said {stop:?}"
	);
	assert!(machine.asked.borrow().is_empty());
}

#[tokio::test]
async fn a_leftover_that_will_not_go_stops_the_run_before_anyone_waits() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);
	let recorder = Recorder::default();
	let clinging = Host {
		clinging: true,
		..HOST
	};
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let mut session = session(PathBuf::from(project.path()), &recorder);

	Execution::open(
		&root,
		&layout,
		"book-a-demo-runtime-0007",
		"book-a-demo-runtime",
	)
	.unwrap();

	session.cluster = &clinging;

	let stop = Run.execute(&mut session).await.unwrap_err();
	let printed = recorder.lines.borrow();

	assert!(
		stop.sentence.contains("is still in the cluster"),
		"said {stop:?}"
	);
	assert_eq!(
		stop.sentence.matches("is still in the cluster").count(),
		1,
		"said {stop:?}"
	);
	assert_eq!(
		printed.last().map(|line| line.id.as_str()),
		Some("gateOpened")
	);
	assert!(!folder.join("usage.json").exists());
}

#[tokio::test]
async fn a_branch_that_cannot_start_stops_the_run_before_anyone_waits() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let recorder = Recorder::default();
	let refusing = Forged {
		refusing: Some("start"),
		..FORGED
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.forge = &refusing;

	let stop = Run.execute(&mut session).await.unwrap_err();
	let printed = recorder.lines.borrow();

	assert_eq!(stop.sentence, "git checkout ended with the exit status 1.");
	assert_eq!(
		printed.last().map(|line| line.id.as_str()),
		Some("gateOpened")
	);
	assert!(!project.path().join(EXECUTION).exists());
}

#[tokio::test]
async fn an_interrupted_run_takes_its_runtime_down_and_closes_its_record() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);
	let recorder = Recorder::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let machine = Machine::default();
	let stalling = Forged {
		stalling: true,
		..FORGED
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.answers = &answers;
	session.cluster = &machine;
	session.forge = &stalling;
	session.interrupt = &Hasty;

	let stop = Run.execute(&mut session).await.unwrap_err();
	let printed = recorder.lines.borrow();
	let record =
		decode::<Ran>(&std::fs::read_to_string(folder.join("run.json")).unwrap())
			.unwrap();
	let execution =
		std::fs::read_to_string(project.path().join(EXECUTION)).unwrap();
	let runtime = record.runtime.unwrap();
	let pull = record.delivery.unwrap().pull_request.unwrap();

	assert_eq!(stop.sentence, "nsx was interrupted.");
	assert_eq!(
		printed.last().map(|line| line.id.as_str()),
		Some("pullRequest")
	);
	assert_eq!(record.outcome, Some(Outcome::Stopped));
	assert_eq!(
		(runtime.state.as_str(), runtime.removed, runtime.left),
		("stopped", Some(5), Some(0))
	);
	assert_eq!(pull.state, "draft");
	assert!(machine.spaces.borrow().is_empty());
	assert!(execution.contains("\"settled\": true"), "{execution}");
}

#[tokio::test]
async fn a_second_interrupt_gives_the_take_down_up_and_says_so() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);
	let recorder = Recorder::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let machine = Machine {
		stalling: true,
		..Machine::default()
	};
	let stalling = Forged {
		stalling: true,
		..FORGED
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.answers = &answers;
	session.cluster = &machine;
	session.forge = &stalling;
	session.interrupt = &Hasty;

	let stop = Run.execute(&mut session).await.unwrap_err();
	let record =
		decode::<Ran>(&std::fs::read_to_string(folder.join("run.json")).unwrap())
			.unwrap();
	let execution =
		std::fs::read_to_string(project.path().join(EXECUTION)).unwrap();

	assert_eq!(
		stop.sentence,
		"nsx was interrupted. The runtime's take-down was interrupted too: the next run of this runtime removes what is left."
	);
	assert_eq!(record.outcome, Some(Outcome::Stopped));
	assert_eq!(record.runtime.unwrap().state, "ready");
	assert!(execution.contains("\"settled\": false"), "{execution}");
}

#[tokio::test]
async fn a_run_interrupted_during_a_stage_ends_stopped_and_asks_no_cluster() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);
	let recorder = Recorder::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let machine = Machine::default();
	let stalling = Coder {
		stalling: Some("spec"),
		..CODER
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.answers = &answers;
	session.agent = &stalling;
	session.cluster = &machine;
	session.interrupt = &Hasty;

	let stop = Run.execute(&mut session).await.unwrap_err();
	let printed = recorder.lines.borrow();
	let record =
		decode::<Ran>(&std::fs::read_to_string(folder.join("run.json")).unwrap())
			.unwrap();

	assert_eq!(stop.sentence, "nsx was interrupted.");
	assert_eq!(
		printed.last().map(|line| line.id.as_str()),
		Some("grilling")
	);
	assert_eq!(record.outcome, Some(Outcome::Stopped));
	assert_eq!(record.runtime, None);
	assert!(machine.asked.borrow().is_empty());
	assert!(!project.path().join(EXECUTION).exists());
}

#[tokio::test]
async fn the_checks_are_followed_from_none_listed_to_all_passed() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let recorder = Recorder::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let hub = Hub {
		commit: String::from(COMMIT),
		opened: (19, String::from(TITLE)),
		..Hub::default()
	};
	let named = CHECKS.map(|name| Check {
		name: String::from(name),
		standing: Standing::Running,
	});
	let passed = CHECKS.map(|name| Check {
		name: String::from(name),
		standing: Standing::Passed,
	});
	let mut session = session(PathBuf::from(project.path()), &recorder);

	hub.listings.borrow_mut().extend([
		Vec::new(),
		named.to_vec(),
		named.to_vec(),
		passed.to_vec(),
	]);

	session.answers = &answers;
	session.forge = &hub;

	Run.execute(&mut session).await.unwrap();

	let printed = recorder.lines.borrow();
	let rendered = printed
		.iter()
		.filter(|line| line.id.starts_with("checks"))
		.map(|line| line.render())
		.collect::<Vec<_>>();
	let asked = hub.asked.borrow();

	assert_eq!(
		rendered,
		[
			"checks     build · check · test · guardrails · running",
			"✓ checks     4 of 4 passed",
		]
	);
	assert_eq!(
		*asked,
		[
			"start feat/0031-book-a-demo-runtime",
			"push feat/0031-book-a-demo-runtime",
			"open feat/0031-book-a-demo-runtime",
			"checks 19",
			"checks 19",
			"checks 19",
			"checks 19",
			"ready 19",
		]
	);
}

#[tokio::test]
async fn a_plan_whose_cards_hold_no_ticket_stops_the_run_before_anyone_waits() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let planned = project.path().join(FOLDER).join("plan.json");
	let mut cards = serde_json::from_str::<serde_json::Value>(
		&std::fs::read_to_string(&planned).unwrap(),
	)
	.unwrap();

	cards.as_object_mut().unwrap().remove("ticket");
	std::fs::write(&planned, cards.to_string()).unwrap();

	let (ended, printed) = played(project.path(), CODER, 2).await;
	let stop = ended.unwrap_err();

	assert!(
		stop.sentence.contains("The plan's cards hold no ticket"),
		"said {stop:?}"
	);
	assert_eq!(printed.last().map(String::as_str), Some("gateOpened"));
}

#[rstest]
#[case::a_call_that_fails(
	Coder { failing: Some("spec"), ..CODER },
	"grilling",
	"The coding agent ended spec with the exit status 2.",
	2
)]
#[case::a_stage_that_leaves_no_report(
	Coder { silent: Some("tickets"), ..CODER },
	"toSpec",
	"The tickets stage left no report",
	3
)]
#[case::a_call_that_reports_no_tokens(
	Coder { unmeasured: Some("implement"), ..CODER },
	"hook",
	"The coding agent reported no token usage for implement",
	4
)]
#[tokio::test]
async fn a_stage_that_fails_prints_no_line_and_keeps_what_was_paid(
	#[case] coder: Coder,
	#[case] last: &str,
	#[case] said: &str,
	#[case] paid: usize,
) {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);

	let (ended, printed) = played(project.path(), coder, 2).await;
	let stop = ended.unwrap_err();
	let record =
		decode::<Ran>(&std::fs::read_to_string(folder.join("run.json")).unwrap())
			.unwrap();
	let usage = decode::<Vec<Usage>>(
		&std::fs::read_to_string(folder.join("usage.json")).unwrap(),
	)
	.unwrap();
	let ledger = decode::<Ledger>(
		&std::fs::read_to_string(project.path().join(LEDGER)).unwrap(),
	)
	.unwrap();

	assert!(stop.sentence.contains(said), "said {stop:?}");
	assert_eq!(printed.last().map(String::as_str), Some(last));
	assert!(!printed.iter().any(|id| id == "completed"));
	assert_eq!(usage[0].calls.len(), paid);
	assert_eq!(record.outcome, Some(Outcome::Stopped));
	assert_eq!(ledger.gate.as_deref(), Some("opened"));
	assert_eq!(ledger.observed_cents, None);
}

#[rstest]
#[case::the_usage_record(
	".langdrift/microruntimes/runtimes/book-a-demo-runtime/usage.json",
	"usage.json"
)]
#[case::the_calls_log(
	".langdrift/agent/state/runs/book-a-demo-runtime-0001/stages/planning.log",
	"planning.log"
)]
#[tokio::test]
async fn a_paid_call_one_file_cannot_keep_is_still_kept_by_the_other(
	#[case] blocked: &'static str,
	#[case] named: &str,
) {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);
	let log = project
		.path()
		.join(".langdrift/agent/state/runs/book-a-demo-runtime-0001/stages")
		.join("planning.log");
	let coder = Coder {
		blocking: Some(("planning", blocked)),
		..CODER
	};

	let (ended, printed) = played(project.path(), coder, 2).await;
	let stop = ended.unwrap_err();
	let logged = std::fs::read_to_string(&log);
	let kept = std::fs::read_to_string(folder.join("usage.json"));
	let held = |written: std::io::Result<String>| {
		written.is_ok_and(|text| text.contains("input"))
	};

	assert!(stop.sentence.contains(named), "said {stop:?}");
	assert_eq!(
		printed.last().map(String::as_str),
		Some("confirmed-planning")
	);
	// One of the two is the folder in the way: the other holds the call.
	assert!(held(logged) != held(kept));
}

#[tokio::test]
async fn a_run_without_a_driver_configuration_stops_before_anyone_waits() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);

	std::fs::remove_file(project.path().join(DRIVER)).unwrap();

	let (ended, printed) = played(project.path(), CODER, 2).await;
	let stop = ended.unwrap_err();

	assert!(
		stop.sentence.contains("no driver configuration"),
		"said {stop:?}"
	);
	assert_eq!(printed.last().map(String::as_str), Some("gateOpened"));
	assert!(!folder.join("usage.json").exists());
}

#[tokio::test]
async fn a_usage_record_that_does_not_read_stops_the_run_before_any_call() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);
	let stages = project
		.path()
		.join(".langdrift/agent/state/runs/book-a-demo-runtime-0001/stages");

	std::fs::write(folder.join("usage.json"), "[{").unwrap();

	let (ended, printed) = played(project.path(), CODER, 2).await;
	let stop = ended.unwrap_err();
	let kept = std::fs::read_to_string(folder.join("usage.json")).unwrap();

	assert!(stop.sentence.contains("usage.json"), "said {stop:?}");
	assert_eq!(printed.last().map(String::as_str), Some("gateOpened"));
	assert_eq!(kept, "[{");
	assert!(!stages.exists());
}

#[tokio::test]
async fn a_run_over_its_budget_pauses_even_without_a_driver_configuration() {
	let project = authorized("claude-opus-5-5", "high", &["review"]).await;

	std::fs::remove_file(project.path().join(DRIVER)).unwrap();

	let (ended, printed) = played(project.path(), CODER, 2).await;

	ended.unwrap();

	assert_eq!(printed.last().map(String::as_str), Some("paused"));
}

#[tokio::test]
async fn a_run_over_its_budget_touches_no_branch_and_no_cluster() {
	let project = authorized("claude-opus-5-5", "high", &["review"]).await;
	let recorder = Recorder::default();
	let machine = Machine::default();
	let hub = Hub::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.cluster = &machine;
	session.forge = &hub;

	Run.execute(&mut session).await.unwrap();

	assert!(machine.asked.borrow().is_empty());
	assert!(hub.asked.borrow().is_empty());
	assert!(!project.path().join(EXECUTION).exists());
}

#[tokio::test]
async fn a_dev_ticket_nobody_closes_stops_the_run_with_no_further_call() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);

	let (ended, printed) = played(project.path(), CODER, 1).await;
	let stop = ended.unwrap_err();
	let record =
		decode::<Ran>(&std::fs::read_to_string(folder.join("run.json")).unwrap())
			.unwrap();
	let usage = decode::<Vec<Usage>>(
		&std::fs::read_to_string(folder.join("usage.json")).unwrap(),
	)
	.unwrap();
	let stages = record.stages.unwrap();

	assert!(stop.sentence.contains("the dev ticket"), "said {stop:?}");
	assert_eq!(printed.last().map(String::as_str), Some("devTicketWaiting"));
	assert_eq!(usage[0].calls.len(), 4);
	assert_eq!(stages[4].state, "active");
	assert_eq!(stages[4].closed_by, None);
	assert_eq!(stages[5].state, "pending");
}

#[tokio::test]
async fn a_call_paid_while_the_dev_ticket_waits_stops_the_run_before_closed() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let folder = project.path().join(FOLDER);
	let recorder = Recorder::default();
	let spender = Spender {
		file: folder.join("usage.json"),
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.answers = &spender;
	session.agent = &CODER;

	let stop = Run.execute(&mut session).await.unwrap_err();
	let printed = recorder.lines.borrow();
	let record =
		decode::<Ran>(&std::fs::read_to_string(folder.join("run.json")).unwrap())
			.unwrap();
	let stages = record.stages.unwrap();

	assert!(
		stop
			.sentence
			.contains("paid for 1 stage calls while the dev ticket waited"),
		"said {stop:?}"
	);
	assert_eq!(
		printed.last().map(|line| line.id.as_str()),
		Some("devTicketWaiting")
	);
	assert_eq!(stages[4].state, "active");
	assert_eq!(stages[4].paid_calls, None);
	assert_eq!(stages[5].state, "pending");
}

#[tokio::test]
async fn the_hook_follows_implements_confirmation_when_implement_is_checked() {
	let project =
		authorized("claude-sonnet-5-5", "medium", &["implement", "review"]).await;

	let (ended, printed) = played(project.path(), CODER, 3).await;
	let around = printed
		.iter()
		.skip_while(|id| *id != "toTickets")
		.take(5)
		.map(String::as_str)
		.collect::<Vec<_>>();

	ended.unwrap();

	assert_eq!(
		around,
		[
			"toTickets",
			"waiting-implement",
			"confirmed-implement",
			"hook",
			"implement",
		]
	);
}
