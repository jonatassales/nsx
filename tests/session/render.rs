//! The six shapes, each rendered to a line the goldens hold

use rstest::rstest;

use nsx::session::{kind::Kind, line::Line, shape::Shape};

use crate::session::{
	comment::comment, component::component, goldens::goldens, labeled::labeled,
	narrative::narrative, result::result, status::status,
};

#[rstest]
#[case::comment(
	comment("plan only · nothing is built, started or paid for in this step"),
	"# plan only · nothing is built, started or paid for in this step"
)]
#[case::narrative(
	narrative("tool call: read 0031-TICKET-book-a-demo-runtime"),
	"console › tool call: read 0031-TICKET-book-a-demo-runtime"
)]
#[case::labeled(
	labeled(
		"estimate",
		"$9.50 · 3.5M input · 250K output · list prices · FIN-02"
	),
	"console › estimate   $9.50 · 3.5M input · 250K output · list prices · FIN-02"
)]
#[case::labeled_with_a_label_of_ten_columns(
	labeled("to-tickets", "3 tickets · blocking edges recorded"),
	"console › to-tickets 3 tickets · blocking edges recorded"
)]
#[case::labeled_with_a_label_over_ten_columns(
	labeled("code-review", "2 findings · fixed"),
	"console › code-review 2 findings · fixed"
)]
#[case::result(
	result("read", "4 documents · nothing outside the ticket's links"),
	"✓ read       4 documents · nothing outside the ticket's links"
)]
#[case::result_with_a_two_word_label(
	result("Safe Seat", "met"),
	"✓ Safe Seat  met"
)]
#[case::status(
	status("reads", "harness/guardrails/0001-finops.gr.md · FIN-02 · FIN-04"),
	"reads      harness/guardrails/0001-finops.gr.md · FIN-02 · FIN-04"
)]
#[case::component_with_a_path_of_nineteen_columns(
	component("services/signup-api", "Go · api", "250m · 256 MiB"),
	"✓ services/signup-api   Go · api        250m · 256 MiB"
)]
#[case::component_with_a_short_path(
	component("apps/signup", "Next.js · web", "500m · 1024 MiB"),
	"✓ apps/signup           Next.js · web   500m · 1024 MiB"
)]
fn a_shape_renders_to_its_golden_line(
	#[case] shape: Shape,
	#[case] golden: &str,
) {
	let line = Line {
		id: String::from("case"),
		kind: Kind::Output,
		shape,
	};

	assert_eq!(line.render(), golden);
	assert!(
		goldens().iter().any(|text| text == golden),
		"no golden holds `{golden}`"
	);
}

#[test]
fn a_line_never_ends_with_a_space() {
	let line = Line {
		id: String::from("case"),
		kind: Kind::Output,
		shape: status("checks", ""),
	};

	assert_eq!(line.render(), "checks");
}
