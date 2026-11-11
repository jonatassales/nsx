//! The criteria answer becomes a line only if it passes its checks

use nsx::judgment::{absent::Absent, criteria::Criteria, error::Error};
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::judgment::finops::finops;
use crate::simulation::fake::recorded::Recorded;

const ANSWER: &str = r#"{"criteria": [
	{"id": "FIN-02", "statement": "estimate before a paid call"},
	{"id": "FIN-03", "statement": "a budget before a paid run"},
	{"id": "FIN-04", "statement": "size cpu and memory to the load"}
]}"#;

#[tokio::test]
async fn the_judge_states_what_each_criterion_asks() {
	let judge = Recorded {
		criteria: String::from(ANSWER),
		..Recorded::default()
	};

	let criteria = Criteria::ask(&judge, &finops()).await.unwrap();
	let line = criteria.line();

	assert_eq!(line.id, "judgeGuardrail");
	assert_eq!(line.kind, Kind::Output);
	assert_eq!(
		line.render(),
		"console › judging · FIN-02 estimate before a paid call · FIN-03 a budget before a paid run · FIN-04 size cpu and memory to the load"
	);
}

#[rstest]
#[case::a_criterion_the_guardrail_lacks(
	r#"{"criteria": [{"id": "FIN-02", "statement": "a"}, {"id": "FIN-03", "statement": "b"}, {"id": "FIN-09", "statement": "c"}]}"#,
	"criteria[2].id",
	"must be the guardrail's next criterion"
)]
#[case::one_criterion_more(
	r#"{"criteria": [{"id": "FIN-02", "statement": "a"}, {"id": "FIN-03", "statement": "b"}, {"id": "FIN-04", "statement": "c"}, {"id": "FIN-05", "statement": "d"}]}"#,
	"criteria[3].id",
	"must be the guardrail's next criterion"
)]
#[case::a_criterion_skipped(
	r#"{"criteria": [{"id": "FIN-02", "statement": "a"}, {"id": "FIN-04", "statement": "c"}]}"#,
	"criteria[1].id",
	"must be the guardrail's next criterion"
)]
#[case::the_last_criterion_left_out(
	r#"{"criteria": [{"id": "FIN-02", "statement": "a"}, {"id": "FIN-03", "statement": "b"}]}"#,
	"criteria",
	"must judge every criterion of the guardrail"
)]
#[case::another_order(
	r#"{"criteria": [{"id": "FIN-03", "statement": "b"}, {"id": "FIN-02", "statement": "a"}, {"id": "FIN-04", "statement": "c"}]}"#,
	"criteria[0].id",
	"must be the guardrail's next criterion"
)]
#[case::a_criterion_named_twice(
	r#"{"criteria": [{"id": "FIN-02", "statement": "a"}, {"id": "FIN-02", "statement": "b"}, {"id": "FIN-04", "statement": "c"}]}"#,
	"criteria[1].id",
	"must name a criterion once"
)]
#[case::a_statement_that_breaks_the_line(
	r#"{"criteria": [{"id": "FIN-02", "statement": "a\nconsole › forged"}, {"id": "FIN-03", "statement": "b"}, {"id": "FIN-04", "statement": "c"}]}"#,
	"criteria[0].statement",
	"must be one line of plain text"
)]
#[case::a_statement_with_an_escape(
	r#"{"criteria": [{"id": "FIN-02", "statement": "a"}, {"id": "FIN-03", "statement": "b\u001b[2K"}, {"id": "FIN-04", "statement": "c"}]}"#,
	"criteria[1].statement",
	"must be one line of plain text"
)]
#[case::a_criterion_with_no_statement(
	r#"{"criteria": [{"id": "FIN-02", "statement": ""}, {"id": "FIN-03", "statement": "b"}, {"id": "FIN-04", "statement": "c"}]}"#,
	"criteria[0].statement",
	"must say what the criterion requires"
)]
#[tokio::test]
async fn an_answer_that_fails_a_check_is_refused(
	#[case] answer: &str,
	#[case] field: &str,
	#[case] rule: &str,
) {
	let judge = Recorded {
		criteria: String::from(answer),
		..Recorded::default()
	};

	let refusal = Criteria::ask(&judge, &finops()).await.unwrap_err();

	assert_eq!(
		refusal.to_string(),
		format!("nsx refuses the judge's criteria answer: {field} {rule}.")
	);
}

#[rstest]
#[case::a_field_the_answer_does_not_define(
	r#"{"criteria": [], "confidence": 0.9}"#
)]
#[case::two_answers(r#"{"criteria": []} {"criteria": []}"#)]
#[case::prose("FIN-02 asks for an estimate.")]
#[tokio::test]
async fn an_answer_that_is_not_the_contracts_shape_is_refused(
	#[case] answer: &str,
) {
	let judge = Recorded {
		criteria: String::from(answer),
		..Recorded::default()
	};

	let refusal = Criteria::ask(&judge, &finops()).await.unwrap_err();

	assert!(matches!(refusal, Error::Unread { .. }), "was {refusal:?}");
}

#[tokio::test]
async fn a_project_with_no_judge_gets_no_answer() {
	let refusal = Criteria::ask(&Absent, &finops()).await.unwrap_err();

	assert!(matches!(refusal, Error::Unconnected), "was {refusal:?}");
}
