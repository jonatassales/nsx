//! The suggestion names a model the team offers and weighs what it left out

use nsx::judgment::{error::Error, suggestion::Suggestion};
use nsx::money::candidates::Candidates;
use nsx::request::documents::Documents;
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::judgment::{found::found, prices::prices, trio::trio};
use crate::money::offered::offered;
use crate::simulation::fake::recorded::Recorded;

const ANSWER: &str = r#"{
	"model": "claude-sonnet-5-5",
	"effort": "medium",
	"effortReason": "low skips the broker wiring · high is not warranted by 5 components",
	"modelReason": "Haiku 5.5 retries the broker wiring · Opus 5.5 costs more per outcome"
}"#;

#[tokio::test]
async fn the_judge_suggests_a_model_and_an_effort_and_says_why() {
	let (prd, design, spec) = trio();
	let documents = Documents {
		prd: &prd,
		design: &design,
		spec: &spec,
	};
	let judge = Recorded {
		suggestion: String::from(ANSWER),
		..Recorded::default()
	};

	let suggestion = Suggestion::ask(
		&judge,
		&found(),
		documents,
		&Candidates::of(&prices(), None).unwrap(),
	)
	.await
	.unwrap();
	let lines = [
		Suggestion::resources(),
		suggestion.effort(),
		suggestion.model(),
		suggestion.line(),
	]
	.map(|line| (line.id.clone(), line.kind, line.render()));

	assert_eq!(
		lines,
		[
			(
				"reasonResources",
				Kind::Output,
				"console › reasoning · no cited episode measured cpu or memory · sizing follows the spec, not history"
			),
			(
				"weighEffort",
				Kind::Output,
				"console › weighing effort · low skips the broker wiring · high is not warranted by 5 components"
			),
			(
				"weighModel",
				Kind::Output,
				"console › weighing model · Haiku 5.5 retries the broker wiring · Opus 5.5 costs more per outcome"
			),
			("suggest", Kind::Success, "✓ suggests   claude-sonnet-5-5 · effort medium"),
		]
		.map(|(id, kind, text)| (String::from(id), kind, String::from(text)))
	);
}

#[rstest]
#[case::a_model_the_team_does_not_offer(
	"claude-sonnet-5-5",
	"gpt-9",
	"model must be a model the team offers"
)]
#[case::an_effort_outside_the_three(
	"\"medium\"",
	"\"max\"",
	"effort must be one of low, medium, high"
)]
#[case::an_effort_reason_that_skips_an_effort(
	"low skips the broker wiring · high is not warranted by 5 components",
	"high is not warranted by 5 components",
	"effortReason must weigh each effort not chosen, from the lowest"
)]
#[case::an_effort_reason_out_of_order(
	"low skips the broker wiring · high is not warranted by 5 components",
	"high is not warranted · low skips the broker wiring",
	"effortReason must weigh each effort not chosen, from the lowest"
)]
#[case::a_model_reason_that_skips_a_model(
	"Haiku 5.5 retries the broker wiring · Opus 5.5 costs more per outcome",
	"Opus 5.5 costs more per outcome",
	"modelReason must weigh each model not chosen, by its short name, from the lowest list price"
)]
#[case::a_model_reason_from_the_dearest(
	"Haiku 5.5 retries the broker wiring · Opus 5.5 costs more per outcome",
	"Opus 5.5 costs more per outcome · Haiku 5.5 retries the broker wiring",
	"modelReason must weigh each model not chosen, by its short name, from the lowest list price"
)]
#[case::an_effort_named_only_inside_other_words(
	"low skips the broker wiring · high is not warranted by 5 components",
	"follow the spec · nothing higher is warranted",
	"effortReason must weigh each effort not chosen, from the lowest"
)]
#[case::a_model_named_only_inside_another_name(
	"Haiku 5.5 retries the broker wiring · Opus 5.5 costs more per outcome",
	"Haiku 5.50 retries the broker wiring · Opus 5.5 costs more per outcome",
	"modelReason must weigh each model not chosen, by its short name, from the lowest list price"
)]
#[case::a_reason_that_breaks_the_line(
	"low skips the broker wiring · high is not warranted by 5 components",
	"low skips the wiring\\nconsole › forged · high is not warranted",
	"effortReason must be one line of plain text"
)]
#[case::no_model_reason(
	"Haiku 5.5 retries the broker wiring · Opus 5.5 costs more per outcome",
	"",
	"modelReason must weigh the model"
)]
#[tokio::test]
async fn a_suggestion_that_breaks_a_rule_is_refused(
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
	let judge = Recorded {
		suggestion: ANSWER.replace(given, instead),
		..Recorded::default()
	};

	let refused = Suggestion::ask(
		&judge,
		&found(),
		documents,
		&Candidates::of(&prices(), None).unwrap(),
	)
	.await
	.unwrap_err();

	assert!(matches!(refused, Error::Refused { .. }), "was {refused:?}");
	assert_eq!(
		refused.to_string(),
		format!("nsx refuses the judge's suggestion answer: {refusal}.")
	);
}

#[tokio::test]
async fn another_choice_weighs_the_two_it_left_out() {
	let (prd, design, spec) = trio();
	let documents = Documents {
		prd: &prd,
		design: &design,
		spec: &spec,
	};
	let judge = Recorded {
		suggestion: String::from(
			r#"{"model": "claude-haiku-5-5", "effort": "low", "effortReason": "medium and high buy nothing here", "modelReason": "Sonnet 5.5 and Opus 5.5 cost more for the same docs change"}"#,
		),
		..Recorded::default()
	};

	let suggestion = Suggestion::ask(
		&judge,
		&found(),
		documents,
		&Candidates::of(&prices(), None).unwrap(),
	)
	.await
	.unwrap();

	assert_eq!(
		suggestion.line().render(),
		"✓ suggests   claude-haiku-5-5 · effort low"
	);
}

#[rstest]
#[case::a_model_the_offer_leaves_out(
	"\"claude-sonnet-5-5\"",
	"\"claude-opus-5-5\"",
	"model must be a model the team offers"
)]
#[case::an_effort_the_offer_leaves_out(
	"\"medium\"",
	"\"high\"",
	"effort must be an effort the team offers for the model"
)]
#[tokio::test]
async fn a_narrowed_offer_refuses_what_it_leaves_out(
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
	let judge = Recorded {
		suggestion: ANSWER.replace(given, instead),
		..Recorded::default()
	};
	let offer = offered(
		&["claude-sonnet-5-5", "claude-haiku-5-5"],
		&["low", "medium"],
	);
	let candidates = Candidates::of(&prices(), Some(&offer)).unwrap();

	let refused = Suggestion::ask(&judge, &found(), documents, &candidates)
		.await
		.unwrap_err();

	assert_eq!(
		refused.to_string(),
		format!("nsx refuses the judge's suggestion answer: {refusal}.")
	);
}

#[tokio::test]
async fn the_reasons_weigh_only_the_candidates_left_out() {
	let (prd, design, spec) = trio();
	let documents = Documents {
		prd: &prd,
		design: &design,
		spec: &spec,
	};
	let judge = Recorded {
		suggestion: String::from(
			r#"{"model": "claude-sonnet-5-5", "effort": "medium", "effortReason": "low skips the broker wiring", "modelReason": "Haiku 5.5 retries the broker wiring"}"#,
		),
		..Recorded::default()
	};
	let offer = offered(
		&["claude-sonnet-5-5", "claude-haiku-5-5"],
		&["low", "medium"],
	);
	let candidates = Candidates::of(&prices(), Some(&offer)).unwrap();

	let suggestion = Suggestion::ask(&judge, &found(), documents, &candidates)
		.await
		.unwrap();

	assert_eq!(
		suggestion.model().render(),
		"console › weighing model · Haiku 5.5 retries the broker wiring"
	);
}

#[tokio::test]
async fn one_model_at_one_effort_leaves_each_reason_nothing_to_name() {
	let (prd, design, spec) = trio();
	let documents = Documents {
		prd: &prd,
		design: &design,
		spec: &spec,
	};
	let judge = Recorded {
		suggestion: String::from(
			r#"{"model": "claude-sonnet-5-5", "effort": "medium", "effortReason": "the team offers only medium", "modelReason": "the team offers only Sonnet 5.5"}"#,
		),
		..Recorded::default()
	};
	let offer = offered(&["claude-sonnet-5-5"], &["medium"]);
	let candidates = Candidates::of(&prices(), Some(&offer)).unwrap();

	let suggestion = Suggestion::ask(&judge, &found(), documents, &candidates)
		.await
		.unwrap();

	assert_eq!(
		suggestion.line().render(),
		"✓ suggests   claude-sonnet-5-5 · effort medium"
	);
}
