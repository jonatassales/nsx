//! The team's Play and Pause, as a connected run applies them

use std::cell::RefCell;
use std::path::PathBuf;
use std::time::Duration;

use nsx::command::run::Run;
use nsx::contract::{
	choice::Choice, decode::decode, encode::encode, intent::Intent,
	run::Run as Ran, status::Status,
};
use rstest::rstest;

use crate::console::{play::play, proposed::proposed, statuses::statuses};
use crate::simulation::{
	authorized::{INTENT, authorized},
	diff::diff,
	fake::{
		answers::Answers, console::Console, hasty::Hasty, recorder::Recorder,
		session::session,
	},
	golden::golden,
	pending,
	printed::printed,
};

const RECORD: &str =
	".langdrift/microruntimes/runtimes/book-a-demo-runtime/run.json";

#[tokio::test]
async fn a_connected_run_tells_every_write_and_prints_the_same_lines() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let recorder = Recorder::default();
	let console = Console::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.answers = &answers;
	session.console = &console;

	Run.execute(&mut session).await.unwrap();

	let reports = console.reports.borrow();
	let last = reports.last().unwrap();
	let differences = diff(&golden("run"), &printed(&recorder), &pending::RUN);
	let told = last.run.as_ref().unwrap();

	assert!(differences.is_empty(), "{}", differences.join("\n"));
	assert!(reports.len() > 20, "sent {}", reports.len());
	assert!(reports.iter().all(|report| report.plan.is_some()));
	assert!(reports.iter().all(|report| report.intent.is_some()));
	assert_eq!(last.status, Status::Done);
	assert!(told.delivery.is_some());
	assert!(
		statuses(&console)
			.iter()
			.all(|status| { matches!(status, Status::Workflow | Status::Done) })
	);
}

#[tokio::test]
async fn a_run_whose_console_fell_silent_still_tells_its_end_once_it_answers() {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let recorder = Recorder::default();
	let console = Console {
		silent_from: Some(2),
		..Console::default()
	};
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.answers = &answers;
	session.console = &console;

	Run.execute(&mut session).await.unwrap();

	assert_eq!(statuses(&console).last(), Some(&Status::Done));
}

#[tokio::test]
async fn a_run_waits_for_the_teams_play_and_then_runs_as_the_simulation_does() {
	let (project, intent) = proposed().await;
	let recorder = Recorder::default();
	let console = Console::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	console.requests.borrow_mut().extend([
		None,
		None,
		play(&Choice::of(&intent)),
	]);

	session.answers = &answers;
	session.console = &console;

	Run.execute(&mut session).await.unwrap();

	let differences = diff(&golden("run"), &printed(&recorder), &pending::RUN);
	let written = decode::<Intent>(
		&std::fs::read_to_string(project.path().join(INTENT)).unwrap(),
	)
	.unwrap();
	let reports = console.reports.borrow();
	let held = reports
		.iter()
		.take_while(|report| report.status == Status::Held)
		.collect::<Vec<_>>();

	assert!(differences.is_empty(), "{}", differences.join("\n"));
	assert_eq!(held.len(), 3);
	assert!(held.iter().all(|report| {
		report
			.intent
			.as_ref()
			.is_some_and(|told| told.status == "proposed")
	}));
	assert!(held.iter().all(|report| report.run.is_none()));
	assert_eq!(reports[3].status, Status::Workflow);
	assert_eq!(written.status, "authorized");
	assert_eq!(
		Intent {
			status: String::from("proposed"),
			..written
		},
		intent
	);
}

#[tokio::test]
async fn a_play_pressed_before_the_run_starts_it_at_once() {
	let (project, intent) = proposed().await;
	let recorder = Recorder::default();
	let console = Console::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	console
		.requests
		.borrow_mut()
		.push_back(play(&Choice::of(&intent)));

	session.answers = &answers;
	session.console = &console;

	Run.execute(&mut session).await.unwrap();

	let differences = diff(&golden("run"), &printed(&recorder), &pending::RUN);

	assert!(differences.is_empty(), "{}", differences.join("\n"));
	assert_eq!(statuses(&console)[..2], [Status::Held, Status::Workflow]);
}

#[tokio::test]
async fn a_play_of_another_choice_is_ignored_and_the_run_keeps_waiting() {
	let (project, intent) = proposed().await;
	let recorder = Recorder::default();
	let console = Console::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let other = Choice {
		effort: String::from("high"),
		..Choice::of(&intent)
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	console.requests.borrow_mut().extend([
		play(&other),
		play(&other),
		play(&Choice::of(&intent)),
	]);

	session.answers = &answers;
	session.console = &console;

	Run.execute(&mut session).await.unwrap();

	let told = statuses(&console);

	assert_eq!(told[..3], [Status::Held, Status::Held, Status::Held]);
	assert_eq!(told[3], Status::Workflow);
}

#[tokio::test]
async fn a_choice_edited_while_the_run_waits_is_the_one_played_and_authorized()
{
	let (project, intent) = proposed().await;
	let recorder = Recorder::default();
	let console = Console::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 2].into()),
	};
	let edited = Intent {
		effort: String::from("low"),
		..intent.clone()
	};
	let file = project.path().join(INTENT);
	let mut session = session(PathBuf::from(project.path()), &recorder);

	*console.rewrite.borrow_mut() =
		Some((2, file.clone(), encode(&edited).unwrap()));
	console.requests.borrow_mut().extend([
		None,
		None,
		play(&Choice::of(&intent)),
		play(&Choice::of(&edited)),
	]);

	session.answers = &answers;
	session.console = &console;

	Run.execute(&mut session).await.unwrap();

	let written =
		decode::<Intent>(&std::fs::read_to_string(&file).unwrap()).unwrap();
	let ran = decode::<Ran>(
		&std::fs::read_to_string(project.path().join(RECORD)).unwrap(),
	)
	.unwrap();
	let reports = console.reports.borrow();
	let efforts = reports
		.iter()
		.take(4)
		.map(|report| report.intent.as_ref().unwrap().effort.clone())
		.collect::<Vec<_>>();

	// The file changes as the second report arrives: the third carries the
	// edit, and the Play of the earlier choice it answers is ignored.
	assert_eq!(efforts, ["medium", "medium", "low", "low"]);
	assert_eq!(written.effort, "low");
	assert_eq!(written.status, "authorized");
	assert_eq!(ran.intent.unwrap().effort, "low");
}

#[tokio::test]
async fn a_play_the_check_then_refuses_tells_the_console_the_run_stopped() {
	let (project, intent) = proposed().await;
	let recorder = Recorder::default();
	let console = Console::default();
	let revised = Intent {
		revision: 2,
		..intent
	};
	let file = project.path().join(INTENT);
	let mut session = session(PathBuf::from(project.path()), &recorder);

	std::fs::write(&file, encode(&revised).unwrap()).unwrap();
	console
		.requests
		.borrow_mut()
		.push_back(play(&Choice::of(&revised)));

	session.console = &console;

	let stop = Run.execute(&mut session).await.unwrap_err();

	assert!(stop.sentence.contains("revision 2"), "said {stop:?}");
	assert!(recorder.lines.borrow().is_empty());
	assert_eq!(statuses(&console), [Status::Held, Status::Error]);
}

#[tokio::test]
async fn a_run_interrupted_while_it_waits_prints_nothing_and_stays_proposed() {
	let (project, intent) = proposed().await;
	let recorder = Recorder::default();
	let console = Console::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.console = &console;
	session.interrupt = &Hasty;

	let stop = Run.execute(&mut session).await.unwrap_err();
	let written = decode::<Intent>(
		&std::fs::read_to_string(project.path().join(INTENT)).unwrap(),
	)
	.unwrap();
	let told = statuses(&console);

	assert_eq!(stop.sentence, "nsx was interrupted.");
	assert!(recorder.lines.borrow().is_empty());
	assert_eq!(written, intent);
	assert_eq!(told.last(), Some(&Status::Error));
	assert!(
		told[..told.len() - 1]
			.iter()
			.all(|status| *status == Status::Held)
	);
}

#[tokio::test]
async fn a_console_that_is_down_never_starts_a_waiting_run() {
	let (project, _intent) = proposed().await;
	let recorder = Recorder::default();
	let console = Console {
		down: true,
		..Console::default()
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.console = &console;
	session.interrupt = &Hasty;

	let stop = Run.execute(&mut session).await.unwrap_err();

	assert_eq!(stop.sentence, "nsx was interrupted.");
	assert!(recorder.lines.borrow().is_empty());
	assert!(console.reports.borrow().len() >= 2);
}

#[tokio::test]
async fn a_project_with_no_connection_still_stops_on_a_proposed_intent() {
	let (project, _intent) = proposed().await;
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);

	let stop = Run.execute(&mut session).await.unwrap_err();

	assert!(
		stop.sentence.contains("the team has not authorized it yet"),
		"said {stop:?}"
	);
	assert!(recorder.lines.borrow().is_empty());
}

#[rstest]
#[case::before_spec("spec")]
#[case::before_the_dev_ticket("dev-ticket")]
#[case::before_review("review")]
#[tokio::test]
async fn a_pause_holds_the_run_before_the_stages_work_until_the_play(
	#[case] stage: &'static str,
) {
	let project = authorized(
		"claude-sonnet-5-5",
		"medium",
		&["spec", "tickets", "implement", "review"],
	)
	.await;
	let recorder = Recorder::default();
	let console = Console {
		pause_before: Some(stage),
		plays_after: Some(2),
		..Console::default()
	};
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 8].into()),
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.answers = &answers;
	session.console = &console;

	Run.execute(&mut session).await.unwrap();

	let reports = console.reports.borrow();
	let held = reports
		.iter()
		.filter(|report| report.status == Status::Held)
		.collect::<Vec<_>>();
	let differences = diff(
		&golden("run-all-stages"),
		&printed(&recorder),
		&pending::STAGES,
	);
	let untouched = held.iter().all(|report| {
		report
			.run
			.iter()
			.flat_map(|run| run.stages.iter().flatten())
			.find(|step| step.id == stage)
			.is_some_and(|step| step.state != "active" && step.report.is_none())
	});

	assert!(differences.is_empty(), "{}", differences.join("\n"));
	assert_eq!(held.len(), 2);
	assert!(untouched);
	assert_eq!(reports.last().unwrap().status, Status::Done);
}

#[tokio::test]
async fn a_run_interrupted_while_held_stops_and_tells_the_console() {
	let project = authorized(
		"claude-sonnet-5-5",
		"medium",
		&["spec", "tickets", "implement", "review"],
	)
	.await;
	let recorder = Recorder::default();
	let console = Console {
		pause_before: Some("spec"),
		..Console::default()
	};
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 8].into()),
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);

	session.answers = &answers;
	session.console = &console;
	session.beat = Duration::from_millis(1);
	session.interrupt = &Hasty;

	let stop = Run.execute(&mut session).await.unwrap_err();

	let told = statuses(&console);

	assert_eq!(stop.sentence, "nsx was interrupted.");
	assert_eq!(told.last(), Some(&Status::Error));
}
