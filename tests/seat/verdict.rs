//! The verdict: whether the run kept the person's seat

use nsx::contract::{outcome::Outcome, run::Run};
use nsx::seat::{part::Part, verdict::Verdict};
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::simulation::fake::moment::{MOMENT, Moment};

#[test]
fn a_dev_ticket_closed_by_the_person_with_no_paid_call_meets_safe_seat() {
	let part = Part {
		state: String::from("done"),
		closed_by: Some(String::from("person")),
		paid_calls: Some(0),
	};
	let mut run = Run {
		run: None,
		intent: None,
		stages: None,
		effort: None,
		safe_seat: None,
		runtime: None,
		delivery: None,
		outcome: Some(Outcome::Running),
	};

	let verdict = Verdict::reach(&part, &Moment);
	let line = verdict.line().unwrap();

	verdict.card(&mut run);

	let section = run.safe_seat.unwrap();

	assert!(verdict.met);
	assert_eq!(verdict.decided_at, MOMENT);
	assert_eq!(
		(line.id.as_str(), line.kind),
		("safeSeatMet", Kind::Success)
	);
	assert_eq!(line.render(), "✓ Safe Seat  met");
	assert_eq!(
		(section.safe_seat.as_str(), section.decided_at.as_str()),
		("met", MOMENT)
	);
}

#[rstest]
#[case::closed_after_a_paid_call("done", Some("person"), Some(1))]
#[case::calls_that_were_not_counted("done", Some("person"), None)]
#[case::still_being_written("active", None, None)]
#[case::closed_by_somebody_else("done", Some("agent"), Some(0))]
fn any_other_dev_ticket_does_not(
	#[case] state: &str,
	#[case] closed_by: Option<&str>,
	#[case] paid_calls: Option<u64>,
) {
	let part = Part {
		state: String::from(state),
		closed_by: closed_by.map(String::from),
		paid_calls,
	};
	let mut run = Run {
		run: None,
		intent: None,
		stages: None,
		effort: None,
		safe_seat: None,
		runtime: None,
		delivery: None,
		outcome: Some(Outcome::Running),
	};

	let verdict = Verdict::reach(&part, &Moment);

	verdict.card(&mut run);

	assert!(!verdict.met);
	assert!(verdict.line().is_none());
	assert!(run.safe_seat.is_none());
}
