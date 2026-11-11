//! The cross-check: distinct ports, resolved images, and every collision named

use std::path::PathBuf;

use nsx::components::{
	collisions::Collisions, component::Component, error::Error,
};
use nsx::contract::cards::Cards;
use nsx::request::named::Named;
use nsx::session::kind::Kind;

use crate::components::resolved::resolved;

#[test]
fn components_that_share_nothing_pass_the_cross_check() {
	let components = [
		resolved("services/signup-api", 8080, Some("demo/signup-api:r1")),
		resolved("services/notify-api", 8081, Some("demo/notify-api:r1")),
		resolved("platform/nats", 4222, Some("demo/nats:r1")),
	];

	let collisions = Collisions::check(&components).unwrap();
	let line = collisions.line();

	assert_eq!(
		(collisions.ports, collisions.images, collisions.collisions),
		(3, 3, 0)
	);
	assert_eq!(line.id, "crossCheck");
	assert_eq!(line.kind, Kind::Output);
	assert_eq!(
		line.render(),
		"console › cross-checking · 3 ports distinct · 3 images resolved · no collisions"
	);
}

#[test]
fn two_components_on_one_port_are_a_stop_that_names_both() {
	let components = [
		resolved("services/signup-api", 8080, Some("demo/signup-api:r1")),
		resolved("platform/nats", 4222, Some("demo/nats:r1")),
		resolved("services/notify-api", 8080, Some("demo/notify-api:r1")),
	];

	let refusal = Collisions::check(&components).unwrap_err();

	assert!(matches!(refusal, Error::Collide { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		"The components collide: services/signup-api and services/notify-api share the port 8080."
	);
}

#[test]
fn two_components_on_one_image_are_a_stop_that_names_both() {
	let components = [
		resolved("services/signup-api", 8080, Some("demo/api:r1")),
		resolved("services/notify-api", 8081, Some("demo/api:r1")),
	];

	let refusal = Collisions::check(&components).unwrap_err();

	assert_eq!(
		refusal.to_string(),
		"The components collide: services/signup-api and services/notify-api share the image demo/api:r1."
	);
}

#[test]
fn one_path_named_twice_is_a_stop_that_names_the_path() {
	let components = [
		resolved("services/signup-api", 8080, None),
		resolved("services/signup-api", 8081, None),
	];

	let refusal = Collisions::check(&components).unwrap_err();

	assert_eq!(
		refusal.to_string(),
		"The components collide: services/signup-api and services/signup-api are the same folder."
	);
}

#[test]
fn two_spellings_of_one_folder_are_a_stop_that_names_both() {
	let components = [
		resolved("services/signup-api", 8080, Some("demo/signup-api:r1")),
		Component {
			folder: PathBuf::from("/project/services/signup-api"),
			..resolved("services/Signup-API", 8090, Some("demo/Signup-API:r1"))
		},
	];

	let refusal = Collisions::check(&components).unwrap_err();

	assert_eq!(
		refusal.to_string(),
		"The components collide: services/signup-api and services/Signup-API are the same folder."
	);
}

#[test]
fn every_collision_is_named_paths_first_then_ports_then_images() {
	let components = [
		resolved("services/a", 8080, Some("demo/a:r1")),
		resolved("services/b", 8080, Some("demo/b:r1")),
		resolved("services/c", 9090, Some("demo/b:r1")),
		resolved("services/d", 9090, Some("demo/d:r1")),
		resolved("services/d", 7070, Some("demo/e:r1")),
	];

	let refusal = Collisions::check(&components).unwrap_err();

	assert_eq!(
		refusal.to_string(),
		"The components collide: services/d and services/d are the same folder; services/a and services/b share the port 8080; services/c and services/d share the port 9090; services/b and services/c share the image demo/b:r1."
	);
}

#[test]
fn three_components_on_one_port_are_one_collision() {
	let components = [
		resolved("services/a", 8080, None),
		resolved("services/b", 8080, None),
		resolved("services/c", 8080, None),
	];

	let refusal = Collisions::check(&components).unwrap_err();

	assert_eq!(
		refusal.to_string(),
		"The components collide: services/a and services/b and services/c share the port 8080."
	);
}

#[test]
fn an_image_not_resolved_is_not_counted_and_shares_nothing() {
	let components = [
		resolved("services/a", 8080, None),
		resolved("services/b", 8081, None),
		resolved("services/c", 8082, Some("demo/c:r1")),
	];

	let collisions = Collisions::check(&components).unwrap();

	assert_eq!((collisions.ports, collisions.images), (3, 1));
}

#[test]
fn the_card_gets_each_rows_image_and_the_three_counts() {
	let components = [
		resolved("services/a", 8080, Some("demo/a:r1")),
		resolved("services/b", 8081, Some("demo/b:r1")),
	];
	let named = ["services/a", "services/b"].map(|path| Named {
		path: String::from(path),
		port: None,
	});
	let collisions = Collisions::check(&components).unwrap();
	let mut cards = Cards::default();

	nsx::components::inventory::Inventory::card(&named, &mut cards);
	collisions.card(&components, &mut cards);

	let written = serde_json::to_value(&cards).unwrap();
	let section = &written["plan"]["components"];

	assert_eq!(section["rows"][0]["image"], "demo/a:r1");
	assert_eq!(section["rows"][1]["image"], "demo/b:r1");
	assert_eq!(section["portsDistinct"], 2);
	assert_eq!(section["imagesResolved"], 2);
	assert_eq!(section["collisions"], 0);
}
