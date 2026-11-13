//! The envelope a design doc sets
//!
//! One function: it takes the design doc as `request/` read it and gives back
//! the envelope, with the design doc's path as its source.

use crate::harness::envelope::Envelope;
use crate::request::design::Design;

impl Envelope {
	pub fn of(design: &Design) -> Self {
		Self {
			cpu_millis: design.cpu_millis,
			memory_mib: design.memory_mib,
			source: design.link.path.clone(),
		}
	}
}
