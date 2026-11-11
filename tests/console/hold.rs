//! A hold before a stage's work: no pause, a pause until the play, and a
//! console that cannot be asked

use std::time::Duration;

use nsx::console::{hold::Hold, report::Report};
use nsx::contract::{asked::Asked, status::Status};

use crate::console::{asked::asked, opened::opened, statuses::statuses};
use crate::simulation::fake::console::Console;

#[tokio::test]
async fn a_hold_point_with_no_pause_asks_once_and_lets_the_work_start() {
	let project = tempfile::tempdir().unwrap();
	let console = Console::default();
	let mut report = opened(&project);

	Hold
		.wait(&mut report, &console, (None, None, None), Duration::ZERO)
		.await
		.unwrap();

	assert_eq!(statuses(&console), [Status::Workflow]);
}

#[tokio::test]
async fn a_pause_holds_until_the_play_reporting_held_at_once_and_at_each_beat()
{
	let project = tempfile::tempdir().unwrap();
	let console = Console::default();
	let mut report = opened(&project);

	console.requests.borrow_mut().extend([
		asked(Asked::Pause),
		None,
		None,
		asked(Asked::Play),
	]);

	Hold
		.wait(&mut report, &console, (None, None, None), Duration::ZERO)
		.await
		.unwrap();

	assert_eq!(
		statuses(&console),
		[Status::Workflow, Status::Held, Status::Held, Status::Held]
	);
	assert_eq!(report.status, Status::Workflow);
}

#[tokio::test]
async fn a_hold_point_that_cannot_reach_the_console_is_no_pause() {
	let project = tempfile::tempdir().unwrap();
	let console = Console {
		down: true,
		..Console::default()
	};
	let mut report = Report {
		unavailable: true,
		..opened(&project)
	};

	Hold
		.wait(&mut report, &console, (None, None, None), Duration::ZERO)
		.await
		.unwrap();

	assert_eq!(statuses(&console), [Status::Workflow]);
}

#[tokio::test]
async fn a_held_run_never_resumes_when_the_console_falls_silent() {
	let project = tempfile::tempdir().unwrap();
	let console = Console {
		silent_from: Some(3),
		..Console::default()
	};
	let mut report = opened(&project);

	console.requests.borrow_mut().push_back(asked(Asked::Pause));

	let held = tokio::time::timeout(
		Duration::from_millis(200),
		Hold.wait(
			&mut report,
			&console,
			(None, None, None),
			Duration::from_millis(5),
		),
	)
	.await;

	assert!(held.is_err(), "the hold ended");
	assert!(console.reports.borrow().len() > 3);
	assert_eq!(report.status, Status::Held);
}

#[tokio::test]
async fn a_held_run_never_resumes_on_a_pause_or_on_nothing() {
	let project = tempfile::tempdir().unwrap();
	let console = Console::default();
	let mut report = opened(&project);

	console.requests.borrow_mut().extend([
		asked(Asked::Pause),
		asked(Asked::Pause),
		None,
	]);

	let held = tokio::time::timeout(
		Duration::from_millis(100),
		Hold.wait(
			&mut report,
			&console,
			(None, None, None),
			Duration::from_millis(5),
		),
	)
	.await;

	assert!(held.is_err(), "the hold ended");
}

#[tokio::test]
async fn a_pause_is_told_held_at_once_before_the_first_beat() {
	let project = tempfile::tempdir().unwrap();
	let console = Console::default();
	let mut report = opened(&project);

	console
		.requests
		.borrow_mut()
		.extend([asked(Asked::Pause), asked(Asked::Play)]);

	let held = tokio::time::timeout(
		Duration::from_millis(500),
		Hold.wait(
			&mut report,
			&console,
			(None, None, None),
			Duration::from_secs(60),
		),
	)
	.await;

	assert!(held.is_ok(), "the first held report waited for a beat");
	assert_eq!(statuses(&console), [Status::Workflow, Status::Held]);
}
