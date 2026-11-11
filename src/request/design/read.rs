//! Reading the design doc
//!
//! One function: it takes the project's root and the design doc's link, and
//! gives back the design doc with the envelope and the budget per run.
//!
//! ## Reads and writes
//!
//! - reads the linked file in the client's artifact folder
//!
//! ## Rules
//!
//! - The `**cpuMillis:**` and `**memoryMiB:**` lines give the envelope: whole
//!   numbers, each at least 1.
//! - The `**Budget per run:**` line, a dollar amount, gives the budget.
//! - It is read before the plan's first line, so the Workflow card's header
//!   and spend exist when the panel appears.
//! - A missing, refused or unreadable link, or a missing or malformed field,
//!   stops the plan (the stop rule). No link inside the design doc is
//!   followed.
//!
//! ## Open
//!
//! - Whether LangDrift imposes these field lines on the client's documents is
//!   the owner's decision.
//!
//! Spec 0078.

use crate::files::{
	root::Root,
	text::{DOCUMENT, text},
};
use crate::money::cents::Cents;
use crate::request::{design::Design, error::Error, field::field, link::Link};

impl Design {
	pub fn read(root: &Root, link: &Link) -> Result<Self, Error> {
		let written = text(root, &link.path, DOCUMENT)?;
		let stated = |name: &'static str| {
			field(&written, name).ok_or_else(|| Error::Field {
				path: link.path.clone(),
				field: name,
			})
		};
		let malformed =
			|name: &'static str, value: &str, cause: &str| Error::Malformed {
				path: link.path.clone(),
				field: name,
				value: String::from(value),
				cause: String::from(cause),
			};

		// One side of the envelope: a whole number, at least 1.
		let side = |name: &'static str| {
			let value = stated(name)?;
			let amount = value
				.parse::<i64>()
				.map_err(|cause| malformed(name, value, &cause.to_string()))?;

			if amount < 1 {
				return Err(malformed(name, value, "it must be at least 1"));
			}

			Ok(amount)
		};

		let cpu_millis = side("cpuMillis")?;
		let memory_mib = side("memoryMiB")?;
		let stated_budget = stated("Budget per run")?;
		let budget = Cents::parse(stated_budget).ok_or_else(|| {
			malformed(
				"Budget per run",
				stated_budget,
				"not a dollar amount such as $25.00",
			)
		})?;

		Ok(Self {
			link: link.clone(),
			cpu_millis,
			memory_mib,
			budget,
		})
	}
}
