//! The authorized intent: chosen among the recorded ones, checked before a run

use std::path::PathBuf;

use nsx::command::run::Run;
use nsx::contract::{
	decode::decode, encode::encode, intent::Intent, outcome::Outcome,
	run::Run as Ran,
};
use nsx::files::{layout::Layout, root::Root};
use nsx::handoff::{authorization::Authorization, error::Error};
use nsx::money::ledger::Ledger;
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::money::listed::listed;
use crate::simulation::{
	authorized::{INTENT, authorized},
	fake::{recorder::Recorder, session::session},
};

const FOLDER: &str = ".langdrift/microruntimes/runtimes/book-a-demo-runtime";

#[tokio::test]
async fn the_authorized_intent_is_chosen_checked_and_printed() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);

	let intent = Authorization::choose(&root, &layout).unwrap();
	let authorization =
		Authorization::check(&root, &layout, intent, &listed().0).unwrap();
	let scope = authorization.scope();
	let line = authorization.line();

	assert_eq!((scope.id.as_str(), scope.kind), ("scope", Kind::Comment));
	assert_eq!(
		scope.render(),
		"# run · 0031-TICKET-book-a-demo-runtime · the recorded intent, authorized by the team"
	);
	assert_eq!((line.id.as_str(), line.kind), ("intent", Kind::Output));
	assert_eq!(
		line.render(),
		"console › intent     claude-sonnet-5-5 · effort medium · runtime.md revision 1"
	);
}

#[rstest]
#[case::still_proposed(
	"status",
	"\"proposed\"",
	"nsx will not run 0031-TICKET-book-a-demo-runtime: the team has not authorized it yet."
)]
#[case::a_stale_revision(
	"revision",
	"7",
	"nsx will not run 0031-TICKET-book-a-demo-runtime: it names revision 7 and the draft is at revision 1."
)]
#[case::a_model_the_price_list_lacks(
	"model",
	"\"claude-mini-1\"",
	"nsx will not run 0031-TICKET-book-a-demo-runtime: its model claude-mini-1 is not in the price list."
)]
#[case::an_effort_outside_the_three(
	"effort",
	"\"extreme\"",
	"nsx will not run 0031-TICKET-book-a-demo-runtime: effort must be one of low, medium, high."
)]
#[tokio::test]
async fn an_intent_the_run_may_not_apply_stops_before_any_line(
	#[case] field: &str,
	#[case] value: &str,
	#[case] sentence: &str,
) {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let file = project.path().join(INTENT);
	let mut intent: serde_json::Value =
		serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();

	intent[field] = serde_json::from_str(value).unwrap();
	std::fs::write(&file, intent.to_string()).unwrap();

	let stop = Run.execute(&mut session).await.unwrap_err();

	assert_eq!(stop.sentence, sentence);
	assert!(recorder.lines.borrow().is_empty());
	assert!(!project.path().join(FOLDER).join("run.json").exists());
}

#[tokio::test]
async fn a_project_with_no_intent_stops_before_any_line() {
	let project = authorized("claude-sonnet-5-5", "medium", &[]).await;
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);

	std::fs::remove_file(project.path().join(INTENT)).unwrap();

	let stop = Run.execute(&mut session).await.unwrap_err();

	assert!(
		stop.sentence.contains("found no intent to run"),
		"said {stop:?}"
	);
	assert!(recorder.lines.borrow().is_empty());
}

#[tokio::test]
async fn two_authorized_intents_are_a_stop_that_names_both() {
	let project = authorized("claude-sonnet-5-5", "medium", &[]).await;
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let written = std::fs::read_to_string(project.path().join(INTENT)).unwrap();
	let mut other = decode::<Intent>(&written).unwrap();
	let folder = layout.runtimes.join("notes-runtime");

	other.ticket = String::from("0032-TICKET-notes-runtime");
	other.runtime_id = String::from("notes-runtime");
	std::fs::create_dir_all(&folder).unwrap();
	std::fs::write(folder.join("intent.json"), encode(&other).unwrap()).unwrap();

	let refusal = Authorization::choose(&root, &layout).unwrap_err();

	assert!(matches!(refusal, Error::Several { .. }), "was {refusal:?}");
	assert!(
		refusal
			.to_string()
			.contains("0031-TICKET-book-a-demo-runtime, 0032-TICKET-notes-runtime"),
		"said {refusal}"
	);
}

#[tokio::test]
async fn the_run_opens_its_record_running_with_an_id_and_every_stage_pending() {
	let project = authorized("claude-opus-5-5", "high", &["review"]).await;
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let folder = project.path().join(FOLDER);

	Run.execute(&mut session).await.unwrap();

	let record =
		decode::<Ran>(&std::fs::read_to_string(folder.join("run.json")).unwrap())
			.unwrap();
	let ledger = decode::<Ledger>(
		&std::fs::read_to_string(
			project
				.path()
				.join(".langdrift/finops/ledger/book-a-demo-runtime-0001.ledger.json"),
		)
		.unwrap(),
	)
	.unwrap();
	let stages = record.stages.unwrap();

	assert_eq!(record.run.as_deref(), Some("book-a-demo-runtime-0001"));
	assert_eq!(record.intent.unwrap().model, "claude-opus-5-5");
	assert_eq!(record.outcome, Some(Outcome::Paused));
	assert_eq!(stages.len(), 7);
	assert!(stages.iter().all(|stage| stage.state == "pending"));
	assert_eq!(
		(
			ledger.estimate_cents,
			ledger.reserved_cents,
			ledger.budget_cents
		),
		(3400, 4080, 2500)
	);
	assert_eq!(ledger.gate.as_deref(), Some("paused"));
	assert_eq!(ledger.observed_cents, None);
}

#[tokio::test]
async fn a_second_run_takes_the_next_id() {
	let project = authorized("claude-opus-5-5", "high", &[]).await;
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let folder = project.path().join(FOLDER);

	Run.execute(&mut session).await.unwrap();
	Run.execute(&mut session).await.unwrap();

	let record =
		decode::<Ran>(&std::fs::read_to_string(folder.join("run.json")).unwrap())
			.unwrap();

	assert_eq!(record.run.as_deref(), Some("book-a-demo-runtime-0002"));
}

#[tokio::test]
async fn a_run_nobody_confirms_stops_at_planning_and_says_so_in_its_record() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let folder = project.path().join(FOLDER);

	let stop = Run.execute(&mut session).await.unwrap_err();
	let record =
		decode::<Ran>(&std::fs::read_to_string(folder.join("run.json")).unwrap())
			.unwrap();
	let lines = recorder.lines.borrow();
	let planning = &record.stages.unwrap()[0];

	assert!(stop.sentence.contains("did not confirm planning"));
	assert_eq!(lines.last().unwrap().id, "waiting-planning");
	assert_eq!(record.outcome, Some(Outcome::Stopped));
	assert_eq!(planning.state, "waiting");
	assert_eq!(planning.confirmed_by, None);
}

#[tokio::test]
async fn an_intent_that_names_another_runtime_than_its_folder_is_a_stop() {
	let project = authorized("claude-sonnet-5-5", "medium", &[]).await;
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let written = std::fs::read_to_string(project.path().join(INTENT)).unwrap();
	let folder = layout.runtimes.join("other");

	std::fs::remove_file(project.path().join(INTENT)).unwrap();
	std::fs::create_dir_all(&folder).unwrap();
	std::fs::write(folder.join("intent.json"), written).unwrap();

	let refusal = Authorization::choose(&root, &layout).unwrap_err();

	assert!(
		matches!(refusal, Error::Misplaced { .. }),
		"was {refusal:?}"
	);
	assert!(
		refusal
			.to_string()
			.contains("names another runtime than its folder"),
		"said {refusal}"
	);
}

#[tokio::test]
async fn an_intent_whose_run_completed_is_not_applied_again() {
	let project = authorized("claude-opus-5-5", "high", &[]).await;
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let file = project.path().join(FOLDER).join("run.json");

	Run.execute(&mut session).await.unwrap();

	let mut record =
		decode::<Ran>(&std::fs::read_to_string(&file).unwrap()).unwrap();

	record.outcome = Some(Outcome::Completed);
	std::fs::write(&file, encode(&record).unwrap()).unwrap();

	let stop = Run.execute(&mut session).await.unwrap_err();

	assert!(
		stop.sentence.contains("found no intent to run"),
		"said {stop:?}"
	);
}

#[tokio::test]
async fn an_intent_that_is_a_folder_is_a_stop_never_a_skip() {
	let project = authorized("claude-sonnet-5-5", "medium", &[]).await;
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let file = project.path().join(INTENT);

	std::fs::remove_file(&file).unwrap();
	std::fs::create_dir(&file).unwrap();

	let stop = Run.execute(&mut session).await.unwrap_err();

	assert!(stop.sentence.contains("intent.json"), "said {stop:?}");
	assert!(recorder.lines.borrow().is_empty());
}

#[rstest]
#[case::no_workflow_card(
	"/workflow",
	"null",
	"priced no run of claude-sonnet-5-5"
)]
#[case::a_negative_estimate(
	"/workflow/spend/4/estimateCents",
	"-950",
	"holds a figure nsx cannot use"
)]
#[case::a_negative_reservation(
	"/workflow/spend/4/reservedCents",
	"-5",
	"holds a figure nsx cannot use"
)]
#[case::a_negative_budget(
	"/workflow/budgetCents",
	"-1",
	"holds a figure nsx cannot use"
)]
#[case::tokens_too_many_to_count(
	"/workflow/spend/4/inputThousands",
	"9000000000",
	"holds a figure nsx cannot use"
)]
#[tokio::test]
async fn a_plan_whose_money_the_run_cannot_use_stops_before_any_line(
	#[case] field: &str,
	#[case] value: &str,
	#[case] said: &str,
) {
	let project = authorized("claude-sonnet-5-5", "medium", &[]).await;
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let file = project.path().join(FOLDER).join("plan.json");
	let mut cards: serde_json::Value =
		serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
	let value: serde_json::Value = serde_json::from_str(value).unwrap();

	if value.is_null() {
		cards.as_object_mut().unwrap().remove("workflow");
	} else {
		*cards.pointer_mut(field).unwrap() = value;
	}

	std::fs::write(&file, cards.to_string()).unwrap();

	let stop = Run.execute(&mut session).await.unwrap_err();

	assert!(stop.sentence.contains(said), "said {stop:?}");
	assert!(recorder.lines.borrow().is_empty());
	assert!(!project.path().join(".langdrift/finops/ledger").exists());
	assert!(!project.path().join(FOLDER).join("run.json").exists());
}
