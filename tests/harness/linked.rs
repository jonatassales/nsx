//! The link a ticket carries to the FinOps guardrail

use std::path::PathBuf;

use nsx::request::{kind::GUARDRAIL, link::Link};

pub fn linked() -> Link {
	Link {
		kind: GUARDRAIL,
		name: String::from("0001-finops.gr.md"),
		short: String::from("0001-finops"),
		path: PathBuf::from(".langdrift/harness/guardrails/0001-finops.gr.md"),
	}
}
