//! An episode is read strictly from its record

use std::path::{Path, PathBuf};

use nsx::files::root::Root;
use nsx::rag::{episode::Episode, error::Error};
use rstest::rstest;

use crate::simulation::fixture::fixture;

#[test]
fn a_fixture_episode_reads_with_its_components_and_no_measurement() {
	let record = Path::new(
		".langdrift/microruntimes/episodes/ep-0412-checkout-preview-sizing.json",
	);

	let episode = Episode::load(&fixture(), record).unwrap();

	assert_eq!(episode.id, "ep-0412");
	assert_eq!(episode.slug, "checkout-preview-sizing");
	assert_eq!(episode.components.len(), 5);
	assert_eq!(episode.resource.kind, "Deployment");
	assert_eq!(episode.measured, None);
}

#[rstest]
#[case::no_measured_field(
	r#"{"id": "ep-0001", "slug": "n", "ticket": "t", "intent": "i", "components": [], "resource": {"kind": "Deployment", "count": 0, "replicas": 1}}"#
)]
#[case::a_field_the_record_does_not_define(
	r#"{"id": "ep-0001", "slug": "n", "ticket": "t", "intent": "i", "components": [], "resource": {"kind": "Deployment", "count": 0, "replicas": 1}, "measured": null, "score": 1}"#
)]
fn a_record_that_is_not_an_episode_is_a_stop(#[case] written: &str) {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	std::fs::write(project.path().join("ep-0001-n.json"), written).unwrap();

	let refusal = Episode::load(&root, Path::new("ep-0001-n.json")).unwrap_err();

	assert!(matches!(refusal, Error::Episode { .. }), "was {refusal:?}");
}
