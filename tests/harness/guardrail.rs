//! A guardrail gives its criteria, in table order, and the steps they govern

use std::path::PathBuf;

use nsx::files::root::Root;
use nsx::harness::{error::Error, guardrail::Guardrail};
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::harness::{linked::linked, project::project};
use crate::simulation::fixture::fixture;

#[test]
fn the_fixtures_guardrail_gives_fin_02_fin_03_and_fin_04() {
	let root = fixture();

	let guardrail = Guardrail::load(&root, &linked()).unwrap();
	let criteria = guardrail
		.criteria
		.iter()
		.map(|criterion| {
			(
				criterion.id.as_str(),
				criterion.rule.as_str(),
				criterion.plan,
				criterion.run,
			)
		})
		.collect::<Vec<_>>();

	assert_eq!(
		criteria,
		[
			("FIN-02", "Estimate before a paid call.", true, true),
			("FIN-03", "A budget before a paid run.", false, true),
			("FIN-04", "Size cpu and memory to the load.", true, false),
		]
	);
}

#[test]
fn the_guardrails_line_names_the_criteria_the_plan_answers_to() {
	let root = fixture();
	let guardrail = Guardrail::load(&root, &linked()).unwrap();

	let line = guardrail.line();

	assert_eq!(line.id, "guardrail");
	assert_eq!(line.kind, Kind::Output);
	assert_eq!(
		line.render(),
		"reads      harness/guardrails/0001-finops.gr.md · FIN-02 · FIN-04"
	);
}

#[test]
fn the_columns_are_found_by_their_names() {
	let written = "# Guardrail\n\n| Checked at | ID | What it asks |\n| --- | --- | --- |\n| run | SEC-01 | No secret in a log. |\n";
	let project = project(written);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let guardrail = Guardrail::load(&root, &linked()).unwrap();

	assert_eq!(guardrail.criteria.len(), 1);
	assert_eq!(guardrail.criteria[0].id, "SEC-01");
	assert_eq!(guardrail.criteria[0].rule, "No secret in a log.");
	assert!(guardrail.criteria[0].run && !guardrail.criteria[0].plan);
}

#[rstest]
#[case::no_table("# FinOps guardrail\n\nA review rule.\n")]
#[case::a_table_without_checked_at(
	"| ID | Rule |\n| --- | --- |\n| FIN-02 | Estimate. |\n"
)]
#[case::a_table_without_id(
	"| Rule | Checked at |\n| --- | --- |\n| Estimate. | plan |\n"
)]
#[case::a_table_with_no_row(
	"| ID | Rule | Checked at |\n| --- | --- | --- |\n"
)]
fn a_guardrail_with_no_criteria_table_is_a_stop(#[case] written: &str) {
	let project = project(written);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Guardrail::load(&root, &linked()).unwrap_err();

	assert!(matches!(refusal, Error::NoTable { .. }), "was {refusal:?}");
}

#[rstest]
#[case::no_step("| FIN-02 | Estimate. | |")]
#[case::another_step("| FIN-02 | Estimate. | review |")]
#[case::no_id("| | Estimate. | plan |")]
#[case::a_short_row("| FIN-02 | Estimate. |")]
fn a_row_that_does_not_say_where_it_is_checked_is_a_stop(#[case] row: &str) {
	let written =
		format!("| ID | Rule | Checked at |\n| --- | --- | --- |\n{row}\n");
	let project = project(&written);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Guardrail::load(&root, &linked()).unwrap_err();

	assert!(matches!(refusal, Error::Row { .. }), "was {refusal:?}");
}

#[test]
fn a_guardrail_that_is_not_there_is_a_stop() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Guardrail::load(&root, &linked()).unwrap_err();

	assert!(matches!(refusal, Error::File(_)), "was {refusal:?}");
}

#[test]
fn a_table_without_its_row_of_dashes_is_no_table() {
	let written = "| ID | Rule | Checked at |\n| FIN-02 | Estimate. | plan |\n| FIN-04 | Size. | plan |\n";
	let project = project(written);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Guardrail::load(&root, &linked()).unwrap_err();

	assert!(matches!(refusal, Error::NoTable { .. }), "was {refusal:?}");
}

#[test]
fn a_row_of_dashes_may_align_its_columns() {
	let written = "| ID | Rule | Checked at |\n| :--- | :---: | ---: |\n| FIN-02 | Estimate. | plan |\n";
	let project = project(written);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let guardrail = Guardrail::load(&root, &linked()).unwrap();

	assert_eq!(guardrail.criteria.len(), 1);
	assert_eq!(guardrail.criteria[0].id, "FIN-02");
}
