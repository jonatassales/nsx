//! A component: inspected in its folder, then printed and put on the card

use std::path::PathBuf;

use nsx::components::{component::Component, error::Error};
use nsx::contract::cards::Cards;
use nsx::files::root::Root;
use nsx::request::named::Named;
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::components::{laid::laid, resolved::resolved};
use crate::simulation::project::project;

#[rstest]
#[case::rust(
	"services/signup-api",
	Some(8080),
	"console › tool call: inspect services/signup-api · Cargo.toml · :8080",
	"✓ services/signup-api   Rust · api      250m · 256 MiB",
	("inspectRust", "rust")
)]
#[case::node(
	"services/notify-api",
	None,
	"console › tool call: inspect services/notify-api · package.json · :8081",
	"✓ services/notify-api   Node.js · api   300m · 512 MiB",
	("inspectNode", "node")
)]
#[case::nats(
	"platform/nats",
	None,
	"console › tool call: inspect platform/nats · nats.conf · :4222",
	"✓ platform/nats         NATS · broker   200m · 256 MiB",
	("inspectNats", "nats")
)]
#[case::redis(
	"platform/redis",
	None,
	"console › tool call: inspect platform/redis · redis.conf · :6379",
	"✓ platform/redis        Redis · cache   200m · 256 MiB",
	("inspectRedis", "redis")
)]
#[case::next(
	"apps/signup",
	None,
	"console › tool call: inspect apps/signup · next.config.ts · :3000",
	"✓ apps/signup           Next.js · web   500m · 1024 MiB",
	("inspectNext", "next")
)]
fn each_component_of_the_fixture_prints_its_two_lines(
	#[case] path: &str,
	#[case] port: Option<i64>,
	#[case] inspect: &str,
	#[case] row: &str,
	#[case] ids: (&str, &str),
) {
	let project = project();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let named = Named {
		path: String::from(path),
		port,
	};

	let component = Component::inspect(&root, &named).unwrap();
	let line = component.line();
	let result = component.row();

	assert_eq!((line.id.as_str(), result.id.as_str()), ids);
	assert_eq!((line.kind, result.kind), (Kind::Output, Kind::Success));
	assert_eq!(line.render(), inspect);
	assert_eq!(result.render(), row);
}

#[rstest]
#[case::the_manifest_before_the_stack(
	"nats.conf",
	"port: 4333\n",
	Some(9),
	4333
)]
#[case::the_stack_before_the_spec(
	"nats.conf",
	"server_name: n1\n",
	Some(9),
	4222
)]
#[case::the_spec_last(
	"Cargo.toml",
	"[package]\nname = \"api\"\n",
	Some(8080),
	8080
)]
#[case::a_manifest_port_from_the_environment(
	"package.json",
	r#"{"scripts": {"start": "serve -p $PORT"}}"#,
	Some(8082),
	8082
)]
#[case::another_scripts_port_never_beats_the_specs(
	"package.json",
	r#"{"scripts": {"dev": "vite --port 5173", "start": "node server.js"}}"#,
	Some(8082),
	8082
)]
fn the_port_is_the_manifests_then_the_stacks_then_the_specs(
	#[case] file: &str,
	#[case] written: &str,
	#[case] stated: Option<i64>,
	#[case] port: i64,
) {
	let (_project, root) = laid("services/api", &[(file, written)]);
	let named = Named {
		path: String::from("services/api"),
		port: stated,
	};

	let component = Component::inspect(&root, &named).unwrap();

	assert_eq!(component.port, port);
}

#[rstest]
#[case::no_source_gives_one(None)]
#[case::the_spec_names_a_port_that_is_none(Some(0))]
#[case::the_spec_names_a_port_above_the_range(Some(65536))]
fn a_component_with_no_port_is_a_stop(#[case] stated: Option<i64>) {
	let files = [("Cargo.toml", "[package]\nname = \"api\"\n")];
	let (_project, root) = laid("services/api", &files);
	let named = Named {
		path: String::from("services/api"),
		port: stated,
	};

	let refusal = Component::inspect(&root, &named).unwrap_err();

	assert!(matches!(refusal, Error::Port { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		"nsx found no port for services/api: neither its manifest, its stack nor the spec gives one."
	);
}

#[test]
fn a_manifest_port_nsx_cannot_read_stops_the_plan_at_that_component() {
	let files = [("nats.conf", "port: 4333 clients\n")];
	let (_project, root) = laid("platform/nats", &files);
	let named = Named {
		path: String::from("platform/nats"),
		port: Some(4222),
	};

	let refusal = Component::inspect(&root, &named).unwrap_err();

	assert!(matches!(refusal, Error::Declared { .. }), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("platform/nats/nats.conf"),
		"said {refusal}"
	);
}

#[test]
fn a_package_json_that_is_not_json_is_a_stop_that_names_the_file() {
	let (_project, root) = laid("services/api", &[("package.json", "{")]);
	let named = Named {
		path: String::from("services/api"),
		port: Some(8080),
	};

	let refusal = Component::inspect(&root, &named).unwrap_err();

	assert!(matches!(refusal, Error::Manifest { .. }), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("services/api/package.json"),
		"said {refusal}"
	);
}

#[test]
fn two_spellings_of_one_path_give_one_folder() {
	let files = [("Cargo.toml", "[package]\nname = \"api\"\n")];
	let (project, root) = laid("services/api", &files);
	let inspected = |path: &str| {
		let named = Named {
			path: String::from(path),
			port: Some(8080),
		};

		Component::inspect(&root, &named).unwrap()
	};

	let plain = inspected("services/api");
	let dotted = inspected("./services/api");

	assert_eq!(plain.folder, dotted.folder);
	assert!(plain.folder.ends_with("services/api"));

	let capitals = Named {
		path: String::from("services/API"),
		port: Some(8080),
	};
	let ignores_case = project.path().join("services/API").is_dir();
	let inspected = Component::inspect(&root, &capitals);

	// A disk that ignores case names the one folder; any other has none.
	match inspected {
		Ok(component) => assert_eq!(component.folder, plain.folder),
		Err(refusal) => {
			assert!(!ignores_case, "was {refusal:?}");
			assert!(matches!(refusal, Error::Missing { .. }), "was {refusal:?}");
		}
	}
}

#[test]
fn a_component_fills_the_placeholder_of_its_path() {
	let component = resolved("services/signup-api", 8080, None);
	let named = [
		Named {
			path: String::from("platform/nats"),
			port: None,
		},
		Named {
			path: String::from("services/signup-api"),
			port: Some(8080),
		},
	];
	let mut cards = Cards::default();

	nsx::components::inventory::Inventory::card(&named, &mut cards);
	component.card(&mut cards);

	let written = serde_json::to_value(&cards).unwrap();

	assert_eq!(
		written["plan"]["components"]["rows"],
		serde_json::json!([
			{
				"path": "platform/nats",
			},
			{
				"path": "services/signup-api",
				"manifest": "Cargo.toml",
				"stack": "Rust",
				"role": "api",
				"port": 8080,
				"cpuMillis": 250,
				"memoryMiB": 256,
			},
		])
	);
}

#[test]
fn a_component_with_no_placeholder_gets_a_row_of_its_own() {
	let component = resolved("services/signup-api", 8080, None);
	let mut cards = Cards::default();

	component.card(&mut cards);
	component.card(&mut cards);

	let rows = cards.components().rows.clone().unwrap();

	assert_eq!(rows.len(), 2);
	assert_eq!(rows[1].stack.as_deref(), Some("Rust"));
}
