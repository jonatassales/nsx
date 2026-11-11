//! The effort: recorded before implement, and a gate on it

use nsx::hook::{effort::Effort, error::Error};
use nsx::session::kind::Kind;

#[test]
fn implement_cannot_start_before_the_record_exists() {
	let refusal =
		Effort::permit(None, "claude-sonnet-5-5", "medium").unwrap_err();

	assert!(matches!(refusal, Error::Unrecorded), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		"nsx will not start implement: the run has recorded no effort yet."
	);
}

#[test]
fn a_record_of_the_applied_choice_lets_implement_start() {
	let effort = Effort {
		model: String::from("claude-sonnet-5-5"),
		effort: String::from("medium"),
		suggested_model: Some(String::from("claude-opus-5-5")),
		suggested_effort: Some(String::from("high")),
		recorded_at: String::from("2026-11-11T12:00:00Z"),
	};
	let line = effort.line();

	Effort::permit(Some(&effort), "claude-sonnet-5-5", "medium").unwrap();

	assert_eq!((line.id.as_str(), line.kind), ("hook", Kind::Output));
	assert_eq!(
		line.render(),
		"console › hook       effort recorded · implement unblocked"
	);
}

#[test]
fn a_record_of_another_choice_does_not() {
	let effort = Effort {
		model: String::from("claude-sonnet-5-5"),
		effort: String::from("medium"),
		suggested_model: None,
		suggested_effort: None,
		recorded_at: String::from("2026-11-11T12:00:00Z"),
	};

	let refusal =
		Effort::permit(Some(&effort), "claude-sonnet-5-5", "high").unwrap_err();

	assert!(matches!(refusal, Error::Mismatch { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		"nsx will not start implement: the recorded effort, claude-sonnet-5-5 at medium, is not the applied choice, claude-sonnet-5-5 at high."
	);
}
