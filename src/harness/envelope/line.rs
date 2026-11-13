//! The envelope's line
//!
//! One function: it gives back the line that states the envelope as the plan's
//! bound, with its provenance.
//!
//! ## Simulation
//!
//! - `bounds`: `bounded    cpuMillis 2000 · memoryMiB 3072 · set by the team in the design doc`
//!
//! ## Rules
//!
//! - `cpuMillis` and `memoryMiB` print as the integers the design doc holds.
//! - The provenance always reads "set by the team in the design doc".
//! - The line prints after the judge's line on the criteria and before the
//!   plan's `budget` line.

use crate::harness::envelope::Envelope;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Envelope {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("bounds"),
			kind: Kind::Output,
			shape: Shape::Status {
				label: String::from("bounded"),
				value: format!(
					"cpuMillis {} · memoryMiB {} · set by the team in the design doc",
					self.cpu_millis, self.memory_mib
				),
			},
		}
	}
}
