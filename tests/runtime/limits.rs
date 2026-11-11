//! The limits statement: what `runtime.md` has no field for

use nsx::contract::cards::Cards;
use nsx::runtime::limits::Limits;
use nsx::session::kind::Kind;

#[test]
fn the_plan_warns_that_gpu_and_disk_are_not_fields() {
	let line = Limits.line();

	assert_eq!(line.id, "limits");
	assert_eq!(line.kind, Kind::Warning);
	assert_eq!(
		line.render(),
		"limits     gpu and disk are not runtime.md fields · none requested"
	);
}

#[test]
fn the_card_says_none_is_requested_with_an_empty_list() {
	let mut cards = Cards::default();

	Limits.card(&mut cards);

	let written = serde_json::to_value(&cards).unwrap();

	assert_eq!(
		written["plan"]["sizing"]["limitsRequested"],
		serde_json::json!([])
	);
}
