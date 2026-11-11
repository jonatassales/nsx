//! The proposed intent: written beside the draft, never authorized by nsx

use std::path::PathBuf;

use nsx::contract::{cards::Cards, decode::decode, intent::Intent};
use nsx::files::root::Root;
use nsx::handoff::error::Error;
use nsx::session::kind::Kind;

use crate::handoff::proposed::proposed;

const FOLDER: &str = ".langdrift/microruntimes/runtimes/book-a-demo-runtime";

#[test]
fn the_plan_says_the_draft_was_proposed() {
	let line = proposed(1).line();

	assert_eq!(line.id, "draft");
	assert_eq!(line.kind, Kind::Success);
	assert_eq!(
		line.render(),
		"✓ plan       runtime.md draft, revision 1 · proposed to the console"
	);
}

#[test]
fn the_written_intent_validates_and_is_proposed() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let folder = project.path().join(FOLDER);

	proposed(1)
		.write(&root, &folder.join("intent.json"))
		.unwrap();

	let written = std::fs::read_to_string(folder.join("intent.json")).unwrap();
	let intent = decode::<Intent>(&written).unwrap();

	assert!(intent.validate().is_empty());
	assert_eq!(intent.status, "proposed");
	assert_eq!(intent.ticket, "0031-TICKET-book-a-demo-runtime");
	assert_eq!(intent.runtime_id, "book-a-demo-runtime");
	assert_eq!(intent.revision, 1);
	assert_eq!(
		(intent.model.as_str(), intent.effort.as_str()),
		("claude-sonnet-5-5", "medium")
	);
	assert!(intent.human.planning && intent.human.review);
	assert!(written.ends_with("}\n"));
}

#[test]
fn the_plan_card_gets_pending_and_the_evidence() {
	let mut cards = Cards::default();

	proposed(2).card(&mut cards);

	let written = serde_json::to_value(&cards).unwrap();

	assert_eq!(written["plan"]["authorization"], "pending");
	assert_eq!(
		written["plan"]["evidence"],
		serde_json::json!({
			"file": "runtime.md",
			"state": "draft",
			"revision": 2,
			"proposedTo": "console",
		})
	);
}

#[test]
fn planning_is_a_human_stage_whatever_the_workflow_says() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let folder = project.path().join(FOLDER);
	let mut proposal = proposed(1);

	proposal.choice.human.planning = false;
	proposal.write(&root, &folder.join("intent.json")).unwrap();

	let written = std::fs::read_to_string(folder.join("intent.json")).unwrap();

	assert!(decode::<Intent>(&written).unwrap().human.planning);
}

#[test]
fn an_intent_that_breaks_a_rule_is_refused_and_nothing_is_written() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let folder = project.path().join(FOLDER);
	let mut proposal = proposed(1);

	proposal.choice.effort = String::from("extreme");

	let refusal = proposal
		.write(&root, &folder.join("intent.json"))
		.unwrap_err();

	assert!(matches!(refusal, Error::Invalid { .. }), "was {refusal:?}");
	assert!(
		refusal
			.to_string()
			.contains("effort must be one of low, medium, high"),
		"said {refusal}"
	);
	assert!(!folder.join("intent.json").exists());
}
