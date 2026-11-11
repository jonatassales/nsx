//! A checkpoint: the owner's answer on the run's standard input

use std::cell::RefCell;

use nsx::flow::{checkpoint::Checkpoint, error::Error};
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::simulation::fake::{answers::Answers, silence::Silence};

#[rstest]
#[case::yes("yes")]
#[case::a_letter("y")]
#[case::capitals("YES")]
#[case::spaces_around_it("  Yes \t")]
fn yes_confirms(#[case] entered: &str) {
	let answers = Answers {
		lines: RefCell::new(vec![String::from(entered)].into()),
	};

	let checkpoint = Checkpoint::wait("planning", &answers).unwrap();
	let line = checkpoint.line();

	assert_eq!(line.id, "confirmed-planning");
	assert_eq!(line.kind, Kind::Success);
	assert_eq!(line.render(), "✓ confirmed  planning · by the owner");
}

#[rstest]
#[case::no("no")]
#[case::an_empty_line("")]
#[case::another_word("later")]
#[case::yes_with_more("yes please")]
fn anything_else_declines(#[case] entered: &str) {
	let answers = Answers {
		lines: RefCell::new(vec![String::from(entered)].into()),
	};

	let refusal = Checkpoint::wait("spec", &answers).unwrap_err();

	assert!(matches!(refusal, Error::Declined { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		"The owner did not confirm spec, so the run stops there: answer yes to go on."
	);
}

#[test]
fn the_end_of_the_input_declines() {
	let refusal = Checkpoint::wait("planning", &Silence).unwrap_err();

	assert!(matches!(refusal, Error::Declined { .. }), "was {refusal:?}");
}

#[rstest]
#[case::planning(
	"planning",
	"console › waiting    planning · the owner confirms"
)]
#[case::spec("spec", "console › waiting    spec · the owner confirms")]
#[case::tickets("tickets", "console › waiting    tickets · the owner confirms")]
#[case::implement(
	"implement",
	"console › waiting    implement · the owner confirms"
)]
fn a_waiting_checkpoint_names_its_stage(
	#[case] stage: &str,
	#[case] printed: &str,
) {
	let waiting = Checkpoint::Waiting {
		stage: String::from(stage),
	};
	let line = waiting.line();

	assert_eq!(line.id, format!("waiting-{stage}"));
	assert_eq!(line.kind, Kind::Output);
	assert_eq!(line.render(), printed);
}
