//! What the simulation's team offers: the three models at low, medium and
//! high
//!
//! The offer is the sample kept beside the contract's tests.

use std::path::Path;

use nsx::contract::{decode::decode, offer::Offer};

pub fn offer() -> Offer {
	let sample = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("tests/contract/samples/offer/offer.sample.json");

	decode(&std::fs::read_to_string(sample).unwrap()).unwrap()
}
