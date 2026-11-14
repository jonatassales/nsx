//! Each published sample reads as its shape and writes back unchanged

use nsx::contract::{
	cards::Cards, criteria::Criteria, episodes::Episodes, fence::Fence,
	implement::Implement, intent::Intent, judged::Judged, offer::Offer,
	planning::Planning, reply::Reply, report::Report, review::Review, run::Run,
	spec::Spec, suggestion::Suggestion, tickets::Tickets,
};
use rstest::rstest;

use crate::contract::trip::trip;

#[rstest]
#[case::fence(trip::<Fence>, "configuration/runtime.sample.json")]
#[case::plan(trip::<Cards>, "handoff/plan.sample.json")]
#[case::intent(trip::<Intent>, "handoff/intent.sample.json")]
#[case::run(trip::<Run>, "handoff/run.sample.json")]
#[case::criteria(trip::<Criteria>, "judgment/criteria.answer.sample.json")]
#[case::episodes(trip::<Episodes>, "judgment/episodes.answer.sample.json")]
#[case::suggestion(
	trip::<Suggestion>,
	"judgment/suggestion.answer.sample.json"
)]
#[case::planning(trip::<Planning>, "stages/planning.report.sample.json")]
#[case::spec(trip::<Spec>, "stages/spec.report.sample.json")]
#[case::tickets(trip::<Tickets>, "stages/tickets.report.sample.json")]
#[case::implement(trip::<Implement>, "stages/implement.report.sample.json")]
#[case::review(trip::<Review>, "stages/review.report.sample.json")]
#[case::report(trip::<Report>, "report/report.sample.json")]
#[case::ready(trip::<Report>, "report/ready.report.sample.json")]
#[case::reply(trip::<Reply>, "report/answer.sample.json")]
#[case::reply_with_a_play(trip::<Reply>, "report/answer.play.sample.json")]
#[case::judged(trip::<Judged>, "judge/response.sample.json")]
#[case::offer(trip::<Offer>, "offer/offer.sample.json")]
fn each_sample_reads_and_writes_back_unchanged(
	#[case] trip: fn(&str),
	#[case] name: &str,
) {
	trip(name);
}
