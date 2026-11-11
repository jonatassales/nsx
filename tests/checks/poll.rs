//! The poll ends when every check passed, and stops when one did not

use std::time::Duration;

use nsx::checks::{
	error::Error,
	standing::Standing::{Failed, Passed, Running},
	suite::Suite,
};

use crate::checks::{
	hubbed::hubbed,
	listed::listed,
	running::{NAMES, running},
};
use crate::simulation::fake::forged::COMMIT;

#[tokio::test]
async fn four_passing_checks_end_the_poll() {
	let hub = hubbed();

	hub.listings.borrow_mut().extend([
		listed(&[
			("build", Passed),
			("check", Passed),
			("test", Running),
			("guardrails", Running),
		]),
		listed(&NAMES.map(|name| (name, Passed))),
	]);

	let suite = running()
		.poll(&hub, 19, COMMIT, Duration::ZERO)
		.await
		.unwrap();

	assert!(suite.passed());
	assert_eq!(suite.checks.len(), 4);
	assert_eq!(*hub.asked.borrow(), ["checks 19", "checks 19"]);
}

#[tokio::test]
async fn one_failure_stops_the_poll_as_soon_as_the_forge_says_so() {
	let hub = hubbed();

	hub.listings.borrow_mut().extend([
		listed(&[
			("build", Passed),
			("check", Failed),
			("test", Running),
			("guardrails", Running),
		]),
		listed(&NAMES.map(|name| (name, Passed))),
	]);

	let refusal = running()
		.poll(&hub, 19, COMMIT, Duration::ZERO)
		.await
		.unwrap_err();

	assert!(matches!(refusal, Error::Failed { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		"The check check did not pass, so the run stops: the pull request stays a draft."
	);
	assert_eq!(*hub.asked.borrow(), ["checks 19"]);
}

#[tokio::test]
async fn a_failure_already_listed_asks_nothing_more_of_the_forge() {
	let hub = hubbed();
	let suite = Suite {
		checks: listed(&[("build", Passed), ("test", Failed)]),
	};

	let refusal = suite
		.poll(&hub, 19, COMMIT, Duration::ZERO)
		.await
		.unwrap_err();

	assert!(refusal.to_string().contains("The check test did not pass"));
	assert!(hub.asked.borrow().is_empty());
}

#[tokio::test]
async fn checks_that_already_passed_ask_nothing_more_of_the_forge() {
	let hub = hubbed();
	let suite = Suite {
		checks: listed(&NAMES.map(|name| (name, Passed))),
	};

	let polled = suite
		.clone()
		.poll(&hub, 19, COMMIT, Duration::ZERO)
		.await
		.unwrap();

	assert_eq!(polled, suite);
	assert!(hub.asked.borrow().is_empty());
}

#[tokio::test]
async fn an_answer_that_lists_no_check_decides_nothing() {
	let hub = hubbed();

	hub
		.listings
		.borrow_mut()
		.extend([Vec::new(), listed(&NAMES.map(|name| (name, Passed)))]);

	let suite = running()
		.poll(&hub, 19, COMMIT, Duration::ZERO)
		.await
		.unwrap();

	assert!(suite.passed());
	assert_eq!(hub.asked.borrow().len(), 2);
}

#[tokio::test]
async fn the_forge_is_asked_again_only_after_the_pace_it_was_given() {
	let hub = hubbed();
	let pace = Duration::from_millis(40);
	let began = std::time::Instant::now();

	hub
		.listings
		.borrow_mut()
		.push_back(listed(&NAMES.map(|name| (name, Passed))));

	running().poll(&hub, 19, COMMIT, pace).await.unwrap();

	assert!(began.elapsed() >= pace, "took {:?}", began.elapsed());
	assert_eq!(nsx::checks::suite::INTERVAL, Duration::from_secs(15));
}

#[tokio::test]
async fn a_head_that_moved_past_the_pushed_commit_stops_the_poll() {
	let hub = hubbed();

	hub
		.listings
		.borrow_mut()
		.push_back(listed(&NAMES.map(|name| (name, Passed))));

	let refusal = running()
		.poll(
			&hub,
			19,
			"0000000000000000000000000000000000000000",
			Duration::ZERO,
		)
		.await
		.unwrap_err();

	assert!(
		matches!(refusal, Error::Elsewhere { .. }),
		"was {refusal:?}"
	);
	assert_eq!(
		refusal.to_string(),
		"The pull request #19 is not at the commit the run pushed, so the run has no checks of its own to follow: its head moved."
	);
	assert_eq!(*hub.asked.borrow(), ["checks 19"]);
}
