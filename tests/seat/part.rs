//! The dev ticket: the person's part, waited for and closed

use std::cell::RefCell;

use nsx::seat::error::Error;
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::seat::waiting::waiting;
use crate::simulation::fake::{answers::Answers, silence::Silence};

#[rstest]
#[case::a_word("done")]
#[case::an_empty_line("")]
#[case::anything_at_all("no, wait")]
fn any_line_the_person_enters_says_the_part_is_written(#[case] entered: &str) {
	let answers = Answers {
		lines: RefCell::new(vec![String::from(entered)].into()),
	};

	let written = waiting();
	let line = written.line();

	written.wait(&answers).unwrap();

	assert_eq!(written.state, "active");
	assert_eq!(
		(line.id.as_str(), line.kind),
		("devTicketWaiting", Kind::Output)
	);
	assert_eq!(
		line.render(),
		"console › waiting    dev-ticket · the person writes the reserved part"
	);
}

#[test]
fn the_end_of_the_input_stops_the_run_at_the_dev_ticket() {
	let refusal = waiting().wait(&Silence).unwrap_err();

	assert!(matches!(refusal, Error::Unanswered), "was {refusal:?}");
}

#[test]
fn a_closed_dev_ticket_says_who_closed_it_and_what_was_paid() {
	let closed = waiting().close(0);
	let line = closed.line();

	assert_eq!(closed.state, "done");
	assert_eq!(closed.closed_by.as_deref(), Some("person"));
	assert_eq!(closed.paid_calls, Some(0));
	assert_eq!(
		(line.id.as_str(), line.kind),
		("devTicketClosed", Kind::Success)
	);
	assert_eq!(
		line.render(),
		"✓ closed     dev-ticket · written without an agent · no paid call"
	);
}
