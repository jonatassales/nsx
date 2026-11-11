//! The line on the runtime's price
//!
//! One function: it gives back the plan's line that says the runtime itself
//! has no price source, so that nobody mistakes cpu and memory for money.
//!
//! ## Simulation
//!
//! - `price`: `estimate   runtime has no price source · cpu and memory stay in millicores and MiB`
//! - Card: Plan · PRICE, revealed by this line
//!
//! ## Rules
//!
//! - The line prints in the plan, after the `limits` line.
//! - Its label is `estimate`, but it states no amount. The run's `estimate`
//!   line belongs to the estimate.
//!
//! Spec 0079.

use crate::money::price::Price;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Price {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("price"),
			kind: Kind::Output,
			shape: Shape::Status {
				label: String::from("estimate"),
				value: String::from(
					"runtime has no price source · cpu and memory stay in millicores and MiB",
				),
			},
		}
	}
}
