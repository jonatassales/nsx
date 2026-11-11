//! The FinOps guardrail, as the fixture's file gives it

use std::path::PathBuf;

use nsx::harness::{criterion::Criterion, guardrail::Guardrail};

pub fn finops() -> Guardrail {
	let criterion = |id: &str, rule: &str, plan: bool, run: bool| Criterion {
		id: String::from(id),
		rule: String::from(rule),
		plan,
		run,
	};

	Guardrail {
		path: PathBuf::from(".langdrift/harness/guardrails/0001-finops.gr.md"),
		name: String::from("0001-finops.gr.md"),
		short: String::from("0001-finops"),
		criteria: vec![
			criterion("FIN-02", "Estimate before a paid call.", true, true),
			criterion("FIN-03", "A budget before a paid run.", false, true),
			criterion("FIN-04", "Size cpu and memory to the load.", true, false),
		],
	}
}
