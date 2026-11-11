//! The run's branch: named after the ticket, started, pushed and said

use nsx::delivery::{branch::Branch, error::Error};
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::delivery::ticketed::ticketed;
use crate::evidence::recorded::recorded;
use crate::simulation::fake::{forged::COMMIT, hub::Hub};

const NAME: &str = "feat/0031-book-a-demo-runtime";

#[tokio::test]
async fn the_branch_is_named_after_the_ticket_and_started_on_the_forge() {
	let hub = Hub::default();

	let branch = Branch::start(&hub, &ticketed(Some("0031"))).await.unwrap();

	assert_eq!(branch.name, NAME);
	assert_eq!(branch.commit, None);
	assert_eq!(branch.line(), None);
	assert_eq!(*hub.asked.borrow(), [format!("start {NAME}")]);
}

#[tokio::test]
async fn a_ticket_with_no_number_names_no_branch() {
	let hub = Hub::default();

	let refusal = Branch::start(&hub, &ticketed(None)).await.unwrap_err();

	assert!(
		matches!(refusal, Error::Numberless { .. }),
		"was {refusal:?}"
	);
	assert!(
		refusal
			.to_string()
			.contains("0031-TICKET-book-a-demo-runtime")
	);
	assert!(hub.asked.borrow().is_empty());
}

#[tokio::test]
async fn a_pushed_branch_says_its_name_and_its_short_hash() {
	let hub = Hub {
		commit: String::from(COMMIT),
		..Hub::default()
	};
	let started = Branch {
		name: String::from(NAME),
		commit: None,
	};
	let mut run = recorded();

	let pushed = started.push(&hub).await.unwrap();
	let line = pushed.line().unwrap();

	pushed.card(&mut run);

	let delivery = run.delivery.unwrap();

	assert_eq!(pushed.commit.as_deref(), Some(COMMIT));
	assert_eq!(pushed.short(), Some("3f9c2e1"));
	assert_eq!((line.id.as_str(), line.kind), ("pushed", Kind::Success));
	assert_eq!(
		line.render(),
		"✓ pushed     feat/0031-book-a-demo-runtime · 3f9c2e1"
	);
	assert_eq!(delivery.branch.as_deref(), Some(NAME));
	assert_eq!(delivery.commit.as_deref(), Some("3f9c2e1"));
	assert_eq!(delivery.pull_request, None);
}

#[rstest]
#[case::nothing("")]
#[case::a_short_hash("3f9c2e1")]
#[case::one_character_short("3f9c2e1b5a7d4c6e8f0a1b2c3d4e5f6a7b8c9d0")]
#[case::capitals("3F9C2E1B5A7D4C6E8F0A1B2C3D4E5F6A7B8C9D0E")]
#[case::not_hexadecimal("3f9c2e1b5a7d4c6e8f0a1b2c3d4e5f6a7b8c9d0g")]
#[case::two_lines("3f9c2e1b5a7d4c6e8f0a1b2c3d4e5f6a7b8c9d0e\nmore")]
#[tokio::test]
async fn anything_but_a_whole_hash_is_no_hash(#[case] said: &str) {
	let hub = Hub {
		commit: String::from(said),
		..Hub::default()
	};
	let started = Branch {
		name: String::from(NAME),
		commit: None,
	};

	let refusal = started.push(&hub).await.unwrap_err();

	assert!(matches!(refusal, Error::Unhashed), "was {refusal:?}");
}
