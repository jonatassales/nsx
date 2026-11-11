//! The judge's verdicts on the candidates never contradict the threshold

use nsx::judgment::{error::Error, relevance::Relevance};
use nsx::request::documents::Documents;
use rstest::rstest;

use crate::judgment::{found::found, trio::trio};
use crate::simulation::fake::recorded::Recorded;

const ANSWER: &str = r#"{"episodes": [
	{"id": "ep-0412", "verdict": "kept", "reason": "shares the stack and scale"},
	{"id": "ep-0388", "verdict": "kept", "reason": "shares the stack and scale"},
	{"id": "ep-0351", "verdict": "kept", "reason": "shares the stack and scale"},
	{"id": "ep-0297", "verdict": "dropped", "reason": "docs only"},
	{"id": "ep-0240", "verdict": "dropped", "reason": "other runtime"},
	{"id": "ep-0188", "verdict": "dropped", "reason": "no broker"}
]}"#;

#[tokio::test]
async fn the_judge_says_which_candidates_share_the_stack_and_scale() {
	let (prd, design, spec) = trio();
	let documents = Documents {
		prd: &prd,
		design: &design,
		spec: &spec,
	};
	let judge = Recorded {
		episodes: String::from(ANSWER),
		..Recorded::default()
	};

	let relevance = Relevance::ask(&judge, &found(), documents).await.unwrap();
	let line = relevance.line();

	assert_eq!(line.id, "judgeRelevance");
	assert_eq!(
		line.render(),
		"console › judging · 3 share the stack and scale · 3 differ: other runtime, no broker, docs only"
	);
}

#[rstest]
#[case::an_episode_below_the_threshold_kept(
	r#"{"id": "ep-0297", "verdict": "dropped", "reason": "docs only"}"#,
	r#"{"id": "ep-0297", "verdict": "kept", "reason": "shares the stack and scale"}"#,
	"episodes[3].verdict must be the threshold's verdict"
)]
#[case::an_episode_above_the_threshold_dropped(
	r#"{"id": "ep-0412", "verdict": "kept", "reason": "shares the stack and scale"}"#,
	r#"{"id": "ep-0412", "verdict": "dropped", "reason": "no broker"}"#,
	"episodes[0].verdict must be the threshold's verdict"
)]
#[case::an_episode_the_search_did_not_find(
	r#"{"id": "ep-0188", "verdict": "dropped", "reason": "no broker"}"#,
	r#"{"id": "ep-0999", "verdict": "dropped", "reason": "no broker"}"#,
	"episodes[5].id must be a candidate of the search"
)]
#[case::a_reason_outside_the_set(
	r#"{"id": "ep-0188", "verdict": "dropped", "reason": "no broker"}"#,
	r#"{"id": "ep-0188", "verdict": "dropped", "reason": "too old"}"#,
	"episodes[5].reason must be the verdict's reason: shares the stack and scale when kept; other runtime, no broker, docs only when dropped"
)]
#[case::a_reason_that_does_not_fit_its_verdict(
	r#"{"id": "ep-0412", "verdict": "kept", "reason": "shares the stack and scale"}"#,
	r#"{"id": "ep-0412", "verdict": "kept", "reason": "docs only"}"#,
	"episodes[0].reason must be the verdict's reason: shares the stack and scale when kept; other runtime, no broker, docs only when dropped"
)]
#[case::an_episode_judged_twice(
	r#"{"id": "ep-0388", "verdict": "kept", "reason": "shares the stack and scale"}"#,
	r#"{"id": "ep-0412", "verdict": "kept", "reason": "shares the stack and scale"}"#,
	"episodes[1].id must judge each episode once"
)]
#[case::a_candidate_left_out(
	",\n\t{\"id\": \"ep-0188\", \"verdict\": \"dropped\", \"reason\": \"no broker\"}",
	"",
	"episodes must judge every candidate of the search"
)]
#[tokio::test]
async fn an_answer_that_disagrees_with_the_search_is_refused(
	#[case] given: &str,
	#[case] instead: &str,
	#[case] refusal: &str,
) {
	let (prd, design, spec) = trio();
	let documents = Documents {
		prd: &prd,
		design: &design,
		spec: &spec,
	};
	let answer = ANSWER.replace(given, instead);
	let judge = Recorded {
		episodes: answer,
		..Recorded::default()
	};

	let refused = Relevance::ask(&judge, &found(), documents)
		.await
		.unwrap_err();

	assert!(matches!(refused, Error::Refused { .. }), "was {refused:?}");
	assert_eq!(
		refused.to_string(),
		format!("nsx refuses the judge's episodes answer: {refusal}.")
	);
}

#[tokio::test]
async fn a_search_that_drops_nothing_lists_no_reason() {
	let (prd, design, spec) = trio();
	let documents = Documents {
		prd: &prd,
		design: &design,
		spec: &spec,
	};
	let mut retrieval = found();
	let judge = Recorded {
		episodes: String::from(
			r#"{"episodes": [{"id": "ep-0412", "verdict": "kept", "reason": "shares the stack and scale"}]}"#,
		),
		..Recorded::default()
	};

	retrieval.candidates.truncate(1);

	let relevance = Relevance::ask(&judge, &retrieval, documents).await.unwrap();

	assert_eq!(
		relevance.line().render(),
		"console › judging · 1 share the stack and scale · 0 differ"
	);
}
