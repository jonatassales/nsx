//! The report to the console and the console's reply read strictly

use nsx::contract::{
	asked::Asked, decode::decode, reply::Reply, report::Report, status::Status,
};

use crate::contract::sample::sample;

#[test]
fn the_first_report_carries_only_its_status() {
	let report =
		decode::<Report>(&sample("report/ready.report.sample.json")).unwrap();

	assert_eq!(report.status, Status::Ready);
	assert_eq!(report.runtime_id, None);
	assert_eq!(report.plan, None);
	assert_eq!(report.intent, None);
	assert_eq!(report.run, None);
}

#[test]
fn a_status_outside_the_eight_is_refused() {
	let held = decode::<Report>(r#"{"status": "held"}"#).unwrap();

	assert_eq!(held.status, Status::Held);
	assert!(decode::<Report>(r#"{"status": "waiting"}"#).is_err());
	assert!(decode::<Report>(r#"{}"#).is_err());
}

#[test]
fn a_record_with_a_field_it_does_not_define_is_refused() {
	let text = r#"{"status": "plan", "plan": {"budget": {}}}"#;

	assert!(decode::<Report>(text).is_err());
}

#[test]
fn the_reply_carries_the_runtimes_id() {
	let reply = decode::<Reply>(&sample("report/answer.sample.json")).unwrap();

	assert!(!reply.runtime_id.is_empty());
	assert_eq!(reply.requested, None);
	assert!(decode::<Reply>("{}").is_err());
}

#[test]
fn a_reply_carries_a_play_with_its_choice_or_a_pause_alone() {
	let played =
		decode::<Reply>(&sample("report/answer.play.sample.json")).unwrap();
	let paused = decode::<Reply>(
		r#"{
			"runtimeId": "019a6f2e-7c41-7b3a-9d2e-5f8a1c3b4d6e",
			"requested": {"state": "pause"}
		}"#,
	)
	.unwrap();
	let play = played.requested.unwrap();
	let choice = play.intent.unwrap();

	assert_eq!(play.state, Asked::Play);
	assert_eq!(
		(
			choice.revision,
			choice.model.as_str(),
			choice.effort.as_str()
		),
		(1, "claude-sonnet-5-5", "medium")
	);
	assert!(choice.human.planning && choice.human.review);
	assert_eq!(paused.requested.unwrap().state, Asked::Pause);
}

#[test]
fn a_request_it_does_not_define_is_refused() {
	let id = "019a6f2e-7c41-7b3a-9d2e-5f8a1c3b4d6e";
	let refused = [
		r#"{"state": "stop"}"#,
		r#"{"state": "pause", "userId": "someone"}"#,
		r#"{"state": "play", "intent": {"revision": 1}}"#,
	];

	for requested in refused {
		let text = format!(r#"{{"runtimeId": "{id}", "requested": {requested}}}"#);

		assert!(decode::<Reply>(&text).is_err(), "took {requested}");
	}
}
