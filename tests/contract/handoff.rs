//! The hand-off records read strictly, and an intent names every problem

use nsx::contract::{
	cards::Cards, decode::decode, error::Error, intent::Intent, outcome::Outcome,
	run::Run,
};
use rstest::rstest;

use crate::contract::sample::sample;

#[test]
fn a_number_status_or_title_the_ticket_does_not_state_stays_null() {
	let text = r#"{"ticket": {"id": "notes", "number": null, "slug": "notes", "status": null, "title": null}}"#;

	let cards = decode::<Cards>(text).unwrap();
	let header = cards.ticket.unwrap();

	assert_eq!(header.number, None);
	assert_eq!(header.status, None);
	assert_eq!(header.title, None);
}

#[test]
fn a_ticket_field_left_out_is_refused() {
	let text = r#"{"ticket": {"id": "notes", "slug": "notes"}}"#;

	let refusal = decode::<Cards>(text).unwrap_err();

	assert!(matches!(refusal, Error::Record { .. }), "was {refusal:?}");
}

#[rstest]
#[case::a_section_no_card_shows(r#"{"budget": {}}"#)]
#[case::a_field_in_a_section_not_joined_yet(r#"{"workflow": {"stages": []}}"#)]
#[case::a_field_the_evidence_does_not_carry(
	r#"{"plan": {"evidence": {"file": null, "cost": 1}}}"#
)]
fn a_field_a_shape_does_not_define_is_refused(#[case] text: &str) {
	let refusal = decode::<Cards>(text).unwrap_err();

	assert!(matches!(refusal, Error::Record { .. }), "was {refusal:?}");
}

#[test]
fn a_text_with_two_records_is_refused() {
	let refusal = decode::<Cards>("{} {}").unwrap_err();

	assert!(matches!(refusal, Error::Records), "was {refusal:?}");
}

#[test]
fn a_run_not_known_yet_is_null_never_absent() {
	let run = decode::<Run>(r#"{"run": null, "outcome": null}"#).unwrap();

	assert_eq!(run.run, None);
	assert_eq!(run.outcome, None);
	assert_eq!(
		serde_json::to_string(&run).unwrap(),
		r#"{"run":null,"outcome":null}"#
	);
	assert!(decode::<Run>(r#"{"run": null}"#).is_err());
}

#[test]
fn an_outcome_outside_the_four_is_refused() {
	let paused = decode::<Run>(r#"{"run": "r", "outcome": "paused"}"#).unwrap();

	assert_eq!(paused.outcome, Some(Outcome::Paused));
	assert!(decode::<Run>(r#"{"run": "r", "outcome": "failed"}"#).is_err());
}

#[test]
fn the_recorded_intent_breaks_no_rule() {
	let intent = decode::<Intent>(&sample("handoff/intent.sample.json")).unwrap();

	assert_eq!(intent.validate(), Vec::new());
}

#[rstest]
#[case::no_ticket(|intent: &mut Intent| intent.ticket.clear(), "ticket")]
#[case::no_runtime(|intent: &mut Intent| intent.runtime_id.clear(), "runtimeId")]
#[case::revision_0(|intent: &mut Intent| intent.revision = 0, "revision")]
#[case::no_model(|intent: &mut Intent| intent.model.clear(), "model")]
#[case::an_effort_outside_the_three(
	|intent: &mut Intent| intent.effort = String::from("max"),
	"effort"
)]
#[case::planning_without_a_person(
	|intent: &mut Intent| intent.human.planning = false,
	"human.planning"
)]
#[case::a_status_outside_the_two(
	|intent: &mut Intent| intent.status = String::from("held"),
	"status"
)]
fn each_intent_rule_broken_alone_is_one_problem_on_its_field(
	#[case] break_rule: fn(&mut Intent),
	#[case] field: &str,
) {
	let mut intent =
		decode::<Intent>(&sample("handoff/intent.sample.json")).unwrap();

	break_rule(&mut intent);

	let problems = intent.validate();

	assert_eq!(problems.len(), 1, "{problems:?}");
	assert_eq!(problems[0].field, field);
}

#[test]
fn an_intent_names_its_problems_in_field_order() {
	let mut intent =
		decode::<Intent>(&sample("handoff/intent.sample.json")).unwrap();

	intent.status = String::from("held");
	intent.ticket.clear();

	let fields = intent
		.validate()
		.into_iter()
		.map(|problem| problem.field)
		.collect::<Vec<_>>();

	assert_eq!(fields, ["ticket", "status"]);
}

#[rstest]
#[case::planning_whatever_the_choice_says("planning", true)]
#[case::spec("spec", false)]
#[case::tickets("tickets", true)]
#[case::implement("implement", false)]
#[case::review("review", true)]
#[case::the_dev_ticket_has_no_box("dev-ticket", false)]
#[case::merge_has_no_box("merge", false)]
fn a_stage_is_ticked_as_the_choice_says(
	#[case] stage: &str,
	#[case] ticked: bool,
) {
	let human = nsx::contract::human::Human {
		planning: false,
		spec: false,
		tickets: true,
		implement: false,
		review: true,
	};

	assert_eq!(human.ticked(stage), ticked);
}
