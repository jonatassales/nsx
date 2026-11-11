//! The inventory: the line that opens the plan act, and its placeholder rows

use nsx::components::inventory::Inventory;
use nsx::contract::cards::Cards;
use nsx::request::named::Named;
use nsx::session::kind::Kind;

#[test]
fn the_plan_says_how_many_components_the_spec_names() {
	let named = ["services/signup-api", "platform/nats"].map(|path| Named {
		path: String::from(path),
		port: None,
	});

	let line = Inventory::line(&named);

	assert_eq!(line.id, "resolve");
	assert_eq!(line.kind, Kind::Output);
	assert_eq!(
		line.render(),
		"console › resolving 2 components named in the spec · paths, images, ports"
	);
}

#[test]
fn the_card_gets_one_placeholder_row_per_path_in_the_specs_order() {
	let named = ["services/signup-api", "platform/nats"].map(|path| Named {
		path: String::from(path),
		port: None,
	});
	let mut cards = Cards::default();

	Inventory::card(&named, &mut cards);

	let rows = cards.components().rows.clone().unwrap();
	let paths = rows.iter().map(|row| row.path.as_str()).collect::<Vec<_>>();

	assert_eq!(paths, ["services/signup-api", "platform/nats"]);
	assert!(rows.iter().all(|row| row.stack.is_none()));
}
