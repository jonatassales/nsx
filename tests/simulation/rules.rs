//! The diff's own rules, each on a four-line session
//!
//! Ported from the Go golden test's rules (the Go CLI, removed with ticket
//! 0434): identical sessions agree; each disagreement is one message naming
//! its line, however the lines after it shift; a pending line excuses its
//! own text and nothing else, never a kind; and a pending entry that excuses
//! nothing is a disagreement.

use rstest::rstest;

use crate::simulation::{diff::diff, sketch::sketch};

#[rstest]
#[case::identical_sessions_with_nothing_pending_agree(
	&["scope", "intent", "gate", "next"],
	&[],
	&[]
)]
#[case::an_unlisted_line_that_differs(
	&["scope", "other intent", "gate", "next"],
	&[],
	&["line 2, intent, differs"]
)]
#[case::a_listed_line_that_matches(
	&["scope", "intent", "gate", "next"],
	&[("intent", "0079")],
	&["line 2, intent, matches"]
)]
#[case::a_missing_line(
	&["scope", "gate", "next"],
	&[],
	&["line 2, intent, is missing"]
)]
#[case::an_extra_line(
	&["scope", "intent", "early runtime", "gate", "next"],
	&[],
	&["printed line 3, runtimeReady, is extra"]
)]
#[case::a_kind_that_differs(
	&["scope", "intent", "output gate", "next"],
	&[],
	&["line 3, gateOpened, prints as output"]
)]
#[case::a_listed_line_that_differs(
	&["scope", "other intent", "gate", "next"],
	&[("intent", "0079")],
	&[]
)]
#[case::a_listed_line_that_is_missing(
	&["scope", "gate", "next"],
	&[("intent", "0079")],
	&[]
)]
#[case::a_listed_line_whose_kind_differs(
	&["scope", "success intent", "gate", "next"],
	&[("intent", "0079")],
	&["line 2, intent, prints as success"]
)]
#[case::an_extra_line_listed_by_the_id_it_prints_with(
	&["scope", "intent", "early runtime", "gate", "next"],
	&[("runtimeReady", "0081")],
	&[]
)]
#[case::a_listed_line_does_not_excuse_its_neighbour(
	&["scope", "other intent", "other gate", "next"],
	&[("intent", "0079")],
	&["line 3, gateOpened, differs"]
)]
#[case::a_listed_id_excuses_one_extra_line(
	&["scope", "intent", "early runtime", "early runtime", "gate", "next"],
	&[("runtimeReady", "0081")],
	&["printed line 4, runtimeReady, is extra"]
)]
#[case::a_listed_id_that_names_no_line(
	&["scope", "intent", "gate", "next"],
	&[("stage", "0080")],
	&["pending stage, under spec 0080"]
)]
#[case::a_listed_line_that_matches_beside_an_extra_line_of_its_id(
	&["scope", "intent", "intent", "gate", "next"],
	&[("intent", "0079")],
	&["line 2, intent, matches"]
)]
#[case::lines_out_of_order(
	&["scope", "gate", "intent", "next"],
	&[],
	&["line 3, gateOpened, is missing", "printed line 2, gateOpened, is extra"]
)]
fn the_diff_names_each_disagreement_once(
	#[case] printed: &[&str],
	#[case] pending: &[(&str, &str)],
	#[case] naming: &[&str],
) {
	let golden = sketch(&["scope", "intent", "gate", "next"]);

	let differences = diff(&golden, &sketch(printed), pending);

	assert_eq!(differences.len(), naming.len(), "{differences:?}");

	for (difference, name) in differences.iter().zip(naming) {
		assert!(difference.starts_with(name), "{difference}");
	}
}
