//! Each stage's report: accepted when valid, refused when not

use std::path::PathBuf;

use nsx::files::root::Root;
use nsx::session::kind::Kind;
use nsx::skills::{error::Error, report::Report};
use rstest::rstest;

use crate::contract::sample::sample;

const SPEC: &str = ".agents/0011-SPEC-book-a-demo-runtime.spec.md";

const FILE: &str =
	".langdrift/agent/state/runs/notes-0001/stages/stage.report.json";

#[rstest]
#[case::planning(
	"planning",
	"grilling",
	"console › grilling   3 open questions settled"
)]
#[case::spec(
	"spec",
	"toSpec",
	"console › to-spec    0011-SPEC revised · 3 acceptance criteria"
)]
#[case::tickets(
	"tickets",
	"toTickets",
	"console › to-tickets 3 tickets · blocking edges recorded"
)]
#[case::implement(
	"implement",
	"implement",
	"console › implement  tdd · 5 components · 14 files · boundary only · stops at the reserved part"
)]
#[case::review(
	"review",
	"codeReview",
	"console › code-review 2 findings · fixed"
)]
fn a_valid_report_is_accepted_and_gives_the_stages_line(
	#[case] stage: &str,
	#[case] id: &str,
	#[case] printed: &str,
) {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let file = project.path().join(FILE);
	let written = sample(&format!("stages/{stage}.report.sample.json"));

	std::fs::create_dir_all(file.parent().unwrap()).unwrap();
	std::fs::write(&file, written).unwrap();

	let report = Report::check(&root, stage, &file, SPEC).unwrap();
	let line = report.line("0011-SPEC");

	assert_eq!(report.stage, stage);
	assert_eq!((line.id.as_str(), line.kind), (id, Kind::Output));
	assert_eq!(line.render(), printed);
}

#[rstest]
#[case::planning_with_a_field_it_does_not_define(
	"planning",
	r#"{"openQuestionsSettled": 3, "summary": "all good"}"#,
	"summary"
)]
#[case::planning_without_its_count("planning", "{}", "openQuestionsSettled")]
#[case::a_spec_written_rather_than_revised(
	"spec",
	r#"{"spec": ".agents/0011-SPEC-book-a-demo-runtime.spec.md", "revised": false, "acceptanceCriteria": 3}"#,
	"revised must be true"
)]
#[case::another_spec_than_the_ticket_links(
	"spec",
	r#"{"spec": ".agents/0099-SPEC-other.spec.md", "revised": true, "acceptanceCriteria": 3}"#,
	"spec must be the spec the ticket links"
)]
#[case::tickets_without_blocking_edges(
	"tickets",
	r#"{"tickets": 3, "blockingEdgesRecorded": false}"#,
	"blockingEdgesRecorded must be true"
)]
#[case::another_method_than_tdd(
	"implement",
	r#"{"method": "vibes", "components": 5, "files": 14, "boundaryOnly": true, "stoppedAtReservedPart": true}"#,
	"method must be tdd"
)]
#[case::an_implement_past_the_boundary(
	"implement",
	r#"{"method": "tdd", "components": 5, "files": 14, "boundaryOnly": false, "stoppedAtReservedPart": true}"#,
	"boundaryOnly must be true"
)]
#[case::an_implement_that_wrote_the_reserved_part(
	"implement",
	r#"{"method": "tdd", "components": 5, "files": 14, "boundaryOnly": true, "stoppedAtReservedPart": false}"#,
	"stoppedAtReservedPart must be true"
)]
#[case::findings_left_unfixed(
	"review",
	r#"{"findings": 2, "fixed": false}"#,
	"fixed must be true"
)]
#[case::a_report_that_is_not_json("review", "2 findings, fixed", "review")]
#[case::a_negative_count(
	"review",
	r#"{"findings": -1, "fixed": true}"#,
	"review"
)]
fn a_report_that_breaks_its_contract_is_refused(
	#[case] stage: &str,
	#[case] written: &str,
	#[case] said: &str,
) {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let file = project.path().join(FILE);

	std::fs::create_dir_all(file.parent().unwrap()).unwrap();
	std::fs::write(&file, written).unwrap();

	let refusal = Report::check(&root, stage, &file, SPEC).unwrap_err();

	assert!(refusal.to_string().contains(said), "said {refusal}");
	assert!(refusal.to_string().contains(stage), "said {refusal}");
}

#[test]
fn a_stage_that_left_no_report_is_refused() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let file = project.path().join(FILE);

	let refusal = Report::check(&root, "planning", &file, SPEC).unwrap_err();

	assert!(matches!(refusal, Error::Missing { .. }), "was {refusal:?}");
	assert!(
		refusal
			.to_string()
			.contains("The planning stage left no report"),
		"said {refusal}"
	);
}

#[rstest]
#[case::planning(
	"planning",
	r#"{"openQuestionsSettled": 1}"#,
	"console › grilling   1 open question settled"
)]
#[case::spec(
	"spec",
	r#"{"spec": ".agents/0011-SPEC-book-a-demo-runtime.spec.md", "revised": true, "acceptanceCriteria": 1}"#,
	"console › to-spec    0011-SPEC revised · 1 acceptance criterion"
)]
#[case::tickets(
	"tickets",
	r#"{"tickets": 1, "blockingEdgesRecorded": true}"#,
	"console › to-tickets 1 ticket · blocking edges recorded"
)]
#[case::implement(
	"implement",
	r#"{"method": "tdd", "components": 1, "files": 1, "boundaryOnly": true, "stoppedAtReservedPart": true}"#,
	"console › implement  tdd · 1 component · 1 file · boundary only · stops at the reserved part"
)]
#[case::review(
	"review",
	r#"{"findings": 1, "fixed": true}"#,
	"console › code-review 1 finding · fixed"
)]
#[case::no_finding(
	"review",
	r#"{"findings": 0, "fixed": true}"#,
	"console › code-review 0 findings · fixed"
)]
fn a_count_of_one_takes_the_singular(
	#[case] stage: &str,
	#[case] written: &str,
	#[case] printed: &str,
) {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let file = project.path().join(FILE);

	std::fs::create_dir_all(file.parent().unwrap()).unwrap();
	std::fs::write(&file, written).unwrap();

	let report = Report::check(&root, stage, &file, SPEC).unwrap();

	assert_eq!(report.line("0011-SPEC").render(), printed);
}
