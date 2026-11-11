//! Ready means checked and met: anything else leaves the pull request a
//! draft

use nsx::checks::{
	standing::Standing::{self, Failed, Passed, Running},
	suite::Suite,
};
use nsx::delivery::{
	error::Error,
	readiness::Readiness::{self, Draft, Merge, Review},
};
use rstest::rstest;

use crate::delivery::{checked::checked, decided::decided, pulled::pulled};
use crate::simulation::fake::hub::Hub;

#[rstest]
#[case::reviewed(true, Review)]
#[case::not_reviewed(false, Merge)]
#[tokio::test]
async fn a_checked_and_met_pull_request_is_marked_ready(
	#[case] reviewed: bool,
	#[case] state: Readiness,
) {
	let hub = Hub::default();

	let ready = pulled(Draft)
		.ready(&hub, &checked(Passed), &decided(true), reviewed)
		.await
		.unwrap();

	assert_eq!(ready.state, state);
	assert_eq!(ready.number, 19);
	assert_eq!(*hub.asked.borrow(), ["ready 19"]);
}

#[rstest]
#[case::a_failed_check(Failed, true, "its checks have not all passed")]
#[case::a_check_still_running(Running, true, "its checks have not all passed")]
#[case::an_unmet_safe_seat(Passed, false, "Safe Seat is not met")]
#[tokio::test]
async fn a_failed_check_or_an_unmet_safe_seat_leaves_the_pull_request_in_draft(
	#[case] last: Standing,
	#[case] met: bool,
	#[case] reason: &str,
) {
	let hub = Hub::default();

	let refusal = pulled(Draft)
		.ready(&hub, &checked(last), &decided(met), true)
		.await
		.unwrap_err();

	assert!(matches!(refusal, Error::Draft { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		format!("The pull request #19 stays a draft: {reason}.")
	);
	assert!(hub.asked.borrow().is_empty());
}

#[tokio::test]
async fn no_check_at_all_is_not_checked() {
	let hub = Hub::default();
	let nothing = Suite {
		checks: Vec::new(),
	};

	let refusal = pulled(Draft)
		.ready(&hub, &nothing, &decided(true), true)
		.await
		.unwrap_err();

	assert!(matches!(refusal, Error::Draft { .. }), "was {refusal:?}");
	assert!(hub.asked.borrow().is_empty());
}
