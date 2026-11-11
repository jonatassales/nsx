//! The pull request: opened as a draft, said, and kept in the record

use nsx::contract::checks::Checks;
use nsx::delivery::{
	error::Error,
	pull::Pull,
	readiness::Readiness::{self, Draft, Merge, Review},
};
use nsx::money::cents::Cents;
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::delivery::{
	pulled::{TITLE, pulled},
	pushed::pushed,
};
use crate::evidence::recorded::recorded;
use crate::simulation::fake::hub::Hub;

#[tokio::test]
async fn the_pull_request_opens_as_a_draft_with_its_number_and_title() {
	let hub = Hub {
		opened: (19, String::from(TITLE)),
		..Hub::default()
	};

	let opened = Pull::open(&hub, &pushed()).await.unwrap();
	let line = opened.line(Cents(160));

	assert_eq!(opened, pulled(Draft));
	assert_eq!(
		(line.id.as_str(), line.kind),
		("pullRequest", Kind::Success)
	);
	assert_eq!(
		line.render(),
		"✓ pr #19     feat(demo): run book a demo as a microruntime"
	);
	assert_eq!(*hub.asked.borrow(), ["open feat/0031-book-a-demo-runtime"]);
}

#[rstest]
#[case::no_number(0, TITLE)]
#[case::no_title(19, "")]
#[case::a_blank_title(19, "  ")]
#[tokio::test]
async fn a_pull_request_with_no_number_or_title_is_a_stop(
	#[case] number: u64,
	#[case] title: &str,
) {
	let hub = Hub {
		opened: (number, String::from(title)),
		..Hub::default()
	};

	let refusal = Pull::open(&hub, &pushed()).await.unwrap_err();

	assert!(matches!(refusal, Error::Unnamed), "was {refusal:?}");
}

#[rstest]
#[case::for_review(
	Review,
	160,
	"✓ ready      #19 for review · $1.60 released to the envelope"
)]
#[case::to_merge(
	Merge,
	160,
	"✓ ready      #19 to merge · $1.60 released to the envelope"
)]
#[case::another_release(
	Review,
	5,
	"✓ ready      #19 for review · $0.05 released to the envelope"
)]
fn the_ready_line_follows_the_state_and_says_what_was_released(
	#[case] state: Readiness,
	#[case] released: i64,
	#[case] rendered: &str,
) {
	let line = pulled(state).line(Cents(released));

	assert_eq!((line.id.as_str(), line.kind), ("ready", Kind::Success));
	assert_eq!(line.render(), rendered);
}

#[test]
fn the_record_keeps_the_pull_request_and_the_checks_it_already_held() {
	let mut run = recorded();
	let passed = Checks {
		state: String::from("passed"),
		passed: Some(4),
		total: Some(4),
	};

	pushed().card(&mut run);
	pulled(Draft).card(&mut run);

	if let Some(delivery) = run.delivery.as_mut() {
		delivery.pull_request.as_mut().unwrap().checks = Some(passed.clone());
	}

	pulled(Review).card(&mut run);

	let delivery = run.delivery.unwrap();
	let kept = delivery.pull_request.unwrap();

	assert_eq!(delivery.commit.as_deref(), Some("3f9c2e1"));
	assert_eq!((kept.number, kept.title.as_str()), (19, TITLE));
	assert_eq!(kept.state, "ready for review");
	assert_eq!(kept.checks, Some(passed));
}
