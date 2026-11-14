//! A pinned tool, kept verified in the project's tools folder

use std::cell::Cell;
use std::os::unix::fs::PermissionsExt;

use nsx::local::{error::Error, tool::Tool};
use sha2::{Digest, Sha256};

use crate::local::{reached::reached, rooted::rooted};
use crate::simulation::fake::{
	programs::Programs, shelf::Shelf, unreached::Unreached,
};

const BYTES: &[u8] = b"#!/bin/sh\necho a pinned tool\n";

fn pinned(bytes: &[u8]) -> Tool {
	let spelled: String = Sha256::digest(bytes)
		.iter()
		.map(|byte| format!("{byte:02x}"))
		.collect();

	Tool {
		name: "kind",
		url: String::from("https://example.invalid/kind"),
		sha256: Box::leak(spelled.into_boxed_str()),
	}
}

#[tokio::test]
async fn a_missing_tool_is_downloaded_verified_and_written_executable() {
	let folder = tempfile::tempdir().unwrap();
	let shelf = Shelf {
		bytes: BYTES.to_vec(),
		asked: Cell::new(0),
	};

	pinned(BYTES).fetch(folder.path(), &shelf).await.unwrap();

	let kept = folder.path().join("kind");
	let mode = std::fs::metadata(&kept).unwrap().permissions().mode();

	assert_eq!(std::fs::read(&kept).unwrap(), BYTES);
	assert_eq!(mode & 0o777, 0o755);
	assert_eq!(shelf.asked.get(), 1);
}

#[tokio::test]
async fn a_verified_copy_is_kept_without_a_download() {
	let folder = tempfile::tempdir().unwrap();

	std::fs::write(folder.path().join("kind"), BYTES).unwrap();

	pinned(BYTES)
		.fetch(folder.path(), &Unreached)
		.await
		.unwrap();
}

#[tokio::test]
async fn a_copy_that_differs_from_the_pin_is_replaced() {
	let folder = tempfile::tempdir().unwrap();
	let shelf = Shelf {
		bytes: BYTES.to_vec(),
		asked: Cell::new(0),
	};

	std::fs::write(folder.path().join("kind"), b"something else").unwrap();

	pinned(BYTES).fetch(folder.path(), &shelf).await.unwrap();

	assert_eq!(std::fs::read(folder.path().join("kind")).unwrap(), BYTES);
}

#[tokio::test]
async fn downloaded_bytes_that_differ_from_the_pin_are_refused_and_nothing_is_written()
 {
	let folder = tempfile::tempdir().unwrap();
	let shelf = Shelf {
		bytes: b"not the pinned tool".to_vec(),
		asked: Cell::new(0),
	};

	let refusal = pinned(BYTES)
		.fetch(folder.path(), &shelf)
		.await
		.unwrap_err();

	assert!(matches!(
		refusal,
		Error::Unpinned {
			tool: "kind"
		}
	));
	assert_eq!(
		refusal.to_string(),
		"The kind nsx downloaded is not the one it pins: its SHA-256 differs, and nsx will not run it."
	);
	assert_eq!(std::fs::read_dir(folder.path()).unwrap().count(), 0);
}

#[tokio::test]
async fn a_download_that_fails_names_the_tool() {
	let folder = tempfile::tempdir().unwrap();

	let refusal = pinned(BYTES)
		.fetch(folder.path(), &Unreached)
		.await
		.unwrap_err();

	assert_eq!(
		refusal.to_string(),
		"nsx could not download its kind: the web is not reached in tests."
	);
}

#[test]
fn the_pins_are_kind_and_kubectl_from_their_publishers() {
	let pinned = Tool::pinned();
	let names: Vec<&str> = pinned.iter().map(|tool| tool.name).collect();

	assert_eq!(names, ["kind", "kubectl"]);
	assert!(pinned[0].url.starts_with(
		"https://github.com/kubernetes-sigs/kind/releases/download/v0.33.0/kind-"
	));
	assert!(
		pinned[1]
			.url
			.starts_with("https://dl.k8s.io/release/v1.37.2/bin/")
	);
	assert!(pinned.iter().all(|tool| tool.sha256.len() == 64));
}

#[test]
fn a_cluster_tool_starts_with_the_tools_folder_first_on_its_path() {
	let (project, root, _layout) = rooted();
	let programs = Programs::default();
	let tools = project.path().join(".langdrift/agent/state/tools");

	let program = reached(&programs, &root)
		.program("kubectl", &["version"])
		.unwrap();

	let path = &program.environment.variables["PATH"];
	let first = std::env::split_paths(path).next().unwrap();

	assert_eq!(first.canonicalize().ok(), tools.canonicalize().ok());
	assert_eq!(program.name, "kubectl");
}

// On demand only: it downloads the real tools from their publishers, to
// check the pins after a version changes.
#[tokio::test]
#[ignore = "downloads kind and kubectl from the web"]
async fn the_real_pins_download_and_match_their_checksums() {
	let folder = tempfile::tempdir().unwrap();

	for tool in Tool::pinned() {
		tool
			.fetch(folder.path(), &nsx::local::web::Web)
			.await
			.unwrap();
	}

	assert!(folder.path().join("kind").exists());
	assert!(folder.path().join("kubectl").exists());
}
