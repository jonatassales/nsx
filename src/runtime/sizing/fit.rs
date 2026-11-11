//! Fitting the sum inside the bound
//!
//! One function: it takes the sizing of every component and says whether the
//! sum fits the bound the team set, or refuses it.
//!
//! ## Rules
//!
//! - A sum over the bound, in cpu or in memory, stops the plan before the
//!   `size` line, under the stop rule. No draft is written.
//! - The plan asks once, after the cross-check, so the stop's sentence states
//!   the sum of all the components, the bound and the design doc that sets
//!   it.
//! - A sum equal to the bound fits it: the headroom is zero.
//!
//! Spec 0079.

use crate::runtime::{error::Error, sizing::Sizing};

impl Sizing {
	pub fn fit(&self) -> Result<(), Error> {
		let over = self.headroom_cpu_millis < 0 || self.headroom_memory_mib < 0;

		if over {
			return Err(Error::Over {
				cpu_millis: self.cpu_millis,
				memory_mib: self.memory_mib,
				bound_cpu_millis: self.bound.cpu_millis,
				bound_memory_mib: self.bound.memory_mib,
				design: self.bound.source.clone(),
			});
		}

		Ok(())
	}
}
