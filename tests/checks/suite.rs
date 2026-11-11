//! The first listing of the checks, their two lines and their record

use std::time::Duration;

use nsx::checks::{
	error::Error,
	standing::Standing::{Passed, Running},
	suite::Suite,
};
use nsx::contract::{delivery::Delivery, pull::Pull};
use nsx::session::kind::Kind;

use crate::checks::{
	hubbed::hubbed,
	listed::listed,
	running::{NAMES, running},
};
use crate::evidence::recorded::recorded;
use crate::simulation::fake::forged::COMMIT;

#[tokio::test]
async fn the_first_answer_that_lists_a_check_is_the_listing() {
	let hub = hubbed();

	hub.listings.borrow_mut().extend([
		Vec::new(),
		Vec::new(),
		listed(&NAMES.map(|name| (name, Running))),
	]);

	let suite = Suite::list(&hub, 19, COMMIT, Duration::ZERO).await.unwrap();

	assert_eq!(suite.checks.len(), 4);
	assert_eq!(hub.asked.borrow().len(), 3);
}

#[tokio::test]
async fn a_pull_request_that_never_lists_a_check_is_a_stop() {
	let hub = hubbed();

	let refusal = Suite::list(&hub, 19, COMMIT, Duration::ZERO)
		.await
		.unwrap_err();

	assert!(matches!(refusal, Error::Unlisted { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		"The pull request #19 listed no check after 20 answers from the forge: nsx has nothing to follow."
	);
	assert_eq!(hub.asked.borrow().len(), 20);
}

#[tokio::test]
async fn a_pull_request_never_at_the_pushed_commit_says_so_and_not_no_check() {
	let hub = hubbed();

	hub
		.listings
		.borrow_mut()
		.push_back(listed(&NAMES.map(|name| (name, Passed))));

	let refusal = Suite::list(
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
	assert_eq!(hub.asked.borrow().len(), 20);
}

#[test]
fn the_running_line_names_every_listed_check_in_the_forges_order() {
	let line = running().line(false);

	assert_eq!((line.id.as_str(), line.kind), ("checks", Kind::Output));
	assert_eq!(
		line.render(),
		"checks     build · check · test · guardrails · running"
	);
}

#[test]
fn the_passed_line_counts_what_the_forge_listed() {
	let suite = Suite {
		checks: listed(&NAMES.map(|name| (name, Passed))),
	};

	let line = suite.line(true);

	assert_eq!(
		(line.id.as_str(), line.kind),
		("checksPassed", Kind::Success)
	);
	assert_eq!(line.render(), "✓ checks     4 of 4 passed");
}

#[test]
fn a_suite_with_no_check_has_not_passed() {
	let nothing = Suite {
		checks: Vec::new(),
	};
	let some = Suite {
		checks: listed(&[("build", Passed), ("test", Running)]),
	};

	assert!(!nothing.passed());
	assert!(!some.passed());
}

#[test]
fn the_record_keeps_the_state_and_the_counts_only_once_passed() {
	let suite = Suite {
		checks: listed(&NAMES.map(|name| (name, Passed))),
	};
	let opened = Delivery {
		pull_request: Some(Pull {
			number: 19,
			title: String::from("feat: notes"),
			state: String::from("draft"),
			checks: None,
		}),
		..Delivery::default()
	};
	let mut run = recorded();

	run.delivery = Some(opened);

	suite.card(false, &mut run);

	let running = run.delivery.clone().unwrap().pull_request.unwrap().checks;

	suite.card(true, &mut run);

	let passed = run.delivery.unwrap().pull_request.unwrap().checks.unwrap();
	let running = running.unwrap();

	assert_eq!(running.state, "running");
	assert_eq!((running.passed, running.total), (None, None));
	assert_eq!(passed.state, "passed");
	assert_eq!((passed.passed, passed.total), (Some(4), Some(4)));
}

#[test]
fn a_record_with_no_pull_request_keeps_no_checks() {
	let suite = Suite {
		checks: listed(&[("build", Passed)]),
	};
	let mut run = recorded();

	suite.card(true, &mut run);

	assert_eq!(run.delivery, None);
}
