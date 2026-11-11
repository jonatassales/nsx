//! The simulation's three documents, as the plan reads them

use std::path::PathBuf;

use nsx::money::cents::Cents;
use nsx::request::{
	design::Design,
	kind::{DESIGN, Kind, PRD, SPEC},
	link::Link,
	named::Named,
	prd::Prd,
	spec::Spec,
};

pub fn trio() -> (Prd, Design, Spec) {
	let link = |kind: Kind, short: &str, name: &str| Link {
		kind,
		name: String::from(name),
		short: String::from(short),
		path: PathBuf::from(".agents").join(name),
	};
	let prd = Prd {
		link: link(PRD, "0002-PRD", "0002-PRD-book-a-demo.prd.md"),
		use_case: String::from("book-a-demo request"),
	};
	let design = Design {
		link: link(DESIGN, "0005-DD", "0005-DD-book-a-demo-runtime.dd.md"),
		cpu_millis: 2000,
		memory_mib: 3072,
		budget: Cents(2500),
	};
	let spec = Spec {
		link: link(SPEC, "0011-SPEC", "0011-SPEC-book-a-demo-runtime.spec.md"),
		components: vec![Named {
			path: String::from("services/signup-api"),
			port: Some(8080),
		}],
		criteria: 3,
	};

	(prd, design, spec)
}
