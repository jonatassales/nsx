//! The size line
//!
//! One function: it gives back the success line the plan prints once the sum
//! fits the bound: the proposed totals against the bound, the number of
//! one-replica Deployments, and that the figures are the proposed shares.
//!
//! ## Simulation
//!
//! - `size`: `✓ size       1450m of 2000m · 2304 of 3072 MiB · 5 one-replica Deployments · proposed shares`
//! - Card: Plan · HEADROOM, `550m · 768 MiB`
//! - Card: Plan · SUGGESTED RESOURCE, `5 one-replica Deployments`
//!
//! ## Rules
//!
//! - A proposal over the bound never reaches this line: the plan stops
//!   before it (`fit.rs`).

use crate::runtime::sizing::Sizing;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Sizing {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("size"),
			kind: Kind::Success,
			shape: Shape::Result {
				label: String::from("size"),
				value: format!(
					"{}m of {}m · {} of {} MiB · {} one-replica Deployments · proposed shares",
					self.cpu_millis,
					self.bound.cpu_millis,
					self.memory_mib,
					self.bound.memory_mib,
					self.deployments
				),
			},
		}
	}
}
