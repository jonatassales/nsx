//! The envelope is the design doc's, stated as the plan's bound

use std::path::PathBuf;

use nsx::contract::cards::Cards;
use nsx::harness::envelope::Envelope;
use nsx::money::cents::Cents;
use nsx::request::{design::Design, kind::DESIGN, link::Link};

#[test]
fn the_design_docs_envelope_is_the_plans_bound() {
	let design = Design {
		link: Link {
			kind: DESIGN,
			name: String::from("0005-DD-book-a-demo-runtime.dd.md"),
			short: String::from("0005-DD"),
			path: PathBuf::from(".agents/0005-DD-book-a-demo-runtime.dd.md"),
		},
		cpu_millis: 2000,
		memory_mib: 3072,
		budget: Cents(2500),
	};

	let envelope = Envelope::of(&design);

	assert_eq!(
		envelope,
		Envelope {
			cpu_millis: 2000,
			memory_mib: 3072,
			source: PathBuf::from(".agents/0005-DD-book-a-demo-runtime.dd.md"),
		}
	);
	assert_eq!(envelope.line().id, "bounds");
	assert_eq!(
		envelope.line().render(),
		"bounded    cpuMillis 2000 · memoryMiB 3072 · set by the team in the design doc"
	);
}

#[test]
fn the_plan_card_gets_the_bound_with_the_design_doc_as_its_source() {
	let envelope = Envelope {
		cpu_millis: 2000,
		memory_mib: 3072,
		source: PathBuf::from(".agents/0005-DD-book-a-demo-runtime.dd.md"),
	};
	let mut cards = Cards::default();

	envelope.card(&mut cards);

	let written = serde_json::to_value(&cards).unwrap();

	assert_eq!(
		written["plan"]["sizing"]["bound"],
		serde_json::json!({
			"cpuMillis": 2000,
			"memoryMiB": 3072,
			"source": ".agents/0005-DD-book-a-demo-runtime.dd.md",
		})
	);
}
