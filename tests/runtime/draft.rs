//! The draft: made from the plan, validated, and written as `runtime.md`

use std::path::PathBuf;

use nsx::contract::{cards::Cards, fence::Fence};
use nsx::files::root::Root;
use nsx::runtime::{
	draft::Draft, error::Error, revision::Revision, sizing::Sizing,
};
use nsx::session::kind::Kind;

use crate::runtime::{bound::bound, imaged::imaged, suggested::suggested};

const FILE: &str = ".langdrift/microruntimes/runtimes/notes/runtime.md";

#[test]
fn the_draft_carries_the_bound_the_suggestion_and_each_share() {
	let components = imaged();
	let sizing = Sizing::sum(&components, &bound());

	let draft =
		Draft::of("notes", Revision(3), &suggested(), &components, &sizing);
	let fence = &draft.fence;

	assert_eq!((fence.version, fence.revision), (3, 3));
	assert_eq!(fence.runtime_id, "notes");
	assert_eq!((fence.cpu_millis, fence.memory_mib), (2000, 3072));
	assert_eq!(
		(fence.model.as_str(), fence.effort.as_str()),
		("claude-sonnet-5-5", "medium")
	);
	assert_eq!(
		fence
			.components
			.iter()
			.map(|component| {
				(
					component.path.as_str(),
					component.image.as_str(),
					component.cpu_millis,
					component.replicas,
				)
			})
			.collect::<Vec<_>>(),
		[
			("services/api", "notes/api:r1", 250, 1),
			("services/jobs", "notes/jobs:r1", 300, 1),
		]
	);
	assert_eq!(draft.validate().unwrap(), 0);
}

#[test]
fn the_validate_line_states_the_count() {
	let line = Draft::line(0);

	assert_eq!(line.id, "validate");
	assert_eq!(line.kind, Kind::Output);
	assert_eq!(
		line.render(),
		"console › tool call: validate runtime.md draft against the schema · 0 errors"
	);
}

#[test]
fn a_draft_that_breaks_its_rules_is_a_stop_that_counts_and_names_them() {
	let mut components = imaged();

	components[0].image = None;
	components[1].port = 8080;

	let sizing = Sizing::sum(&components, &bound());
	let draft =
		Draft::of("Notes", Revision(1), &suggested(), &components, &sizing);

	let refusal = draft.validate().unwrap_err();

	assert!(matches!(refusal, Error::Invalid { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		"nsx validated the runtime.md draft against the schema and counted 3 errors: runtimeId must be a lowercase DNS label; components[0].image must be an image reference; components[1].port must be distinct."
	);
}

#[test]
fn the_card_gets_the_count_and_the_schema_it_was_counted_against() {
	let mut cards = Cards::default();

	Draft::card(0, &mut cards);

	let written = serde_json::to_value(&cards).unwrap();

	assert_eq!(
		written["plan"]["sizing"]["validation"],
		serde_json::json!({
			"schema": "https://langdrift.md/schemas/configuration/runtime.schema.json",
			"errors": 0,
		})
	);
}

#[test]
fn runtime_md_is_a_title_the_fence_and_the_sizing_report() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let file = project.path().join(FILE);
	let components = imaged();
	let sizing = Sizing::sum(&components, &bound());
	let draft =
		Draft::of("notes", Revision(1), &suggested(), &components, &sizing);

	draft.write(&root, &file, 0).unwrap();

	let written = std::fs::read_to_string(&file).unwrap();
	let (title, rest) = written.split_once("\n\n```langdrift\n").unwrap();
	let (_block, report) = rest.split_once("\n```\n\n").unwrap();

	assert_eq!(title, "# notes");
	assert_eq!(Fence::parse(&written).unwrap(), draft.fence);
	assert_eq!(
		report,
		"## Sizing\n\n| | cpu | memory |\n| --- | --- | --- |\n| Proposed | 550m | 768 MiB |\n| Bound, set by the team in the design doc | 2000m | 3072 MiB |\n| Headroom | 1450m | 2304 MiB |\n\n2 Deployments, 1 replica each. gpu and disk are not runtime.md fields; none requested. Schema validation: 0 errors.\n"
	);
}

#[test]
fn a_later_draft_is_written_over_the_earlier_one() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let file = project.path().join(FILE);
	let components = imaged();
	let sizing = Sizing::sum(&components, &bound());
	let draft = |revision: Revision| {
		Draft::of("notes", revision, &suggested(), &components, &sizing)
	};

	draft(Revision::next(&root, &file).unwrap())
		.write(&root, &file, 0)
		.unwrap();
	draft(Revision::next(&root, &file).unwrap())
		.write(&root, &file, 0)
		.unwrap();

	let written = std::fs::read_to_string(&file).unwrap();

	assert_eq!(Fence::parse(&written).unwrap().revision, 2);
	assert_eq!(Revision::next(&root, &file).unwrap(), Revision(3));
}
