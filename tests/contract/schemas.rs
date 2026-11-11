//! Each schema generated from a type equals its published file
//!
//! The published copies in `schema/` started as Core's files. `plan.json`'s
//! has grown with the cards' fields; the others still equal Core's. They are
//! compared as JSON documents: whitespace and key order do not count.

use nsx::contract::{
	cards::Cards, criteria::Criteria, episodes::Episodes, fence::Fence,
	implement::Implement, intent::Intent, judged::Judged, planning::Planning,
	reply::Reply, report::Report, review::Review, run::Run, schema::schema,
	spec::Spec, suggestion::Suggestion, tickets::Tickets,
};
use pretty_assertions::assert_eq;
use rstest::rstest;
use serde_json::Value;

use crate::contract::published::published;

#[rstest]
#[case::fence(schema::<Fence>(), "configuration/runtime.schema.json")]
#[case::plan(schema::<Cards>(), "handoff/plan.schema.json")]
#[case::intent(schema::<Intent>(), "handoff/intent.schema.json")]
#[case::run(schema::<Run>(), "handoff/run.schema.json")]
#[case::criteria(schema::<Criteria>(), "judgment/criteria.answer.schema.json")]
#[case::episodes(schema::<Episodes>(), "judgment/episodes.answer.schema.json")]
#[case::suggestion(
	schema::<Suggestion>(),
	"judgment/suggestion.answer.schema.json"
)]
#[case::planning(schema::<Planning>(), "stages/planning.report.schema.json")]
#[case::spec(schema::<Spec>(), "stages/spec.report.schema.json")]
#[case::tickets(schema::<Tickets>(), "stages/tickets.report.schema.json")]
#[case::implement(
	schema::<Implement>(),
	"stages/implement.report.schema.json"
)]
#[case::review(schema::<Review>(), "stages/review.report.schema.json")]
#[case::report(schema::<Report>(), "report/report.schema.json")]
#[case::reply(schema::<Reply>(), "report/answer.schema.json")]
#[case::judged(schema::<Judged>(), "judge/response.schema.json")]
fn each_generated_schema_equals_the_published_one(
	#[case] generated: Value,
	#[case] name: &str,
) {
	assert_eq!(generated, published(name));
}
