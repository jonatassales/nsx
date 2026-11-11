//! The one moment

use nsx::evidence::clock::Clock;

use crate::simulation::fake::moment::{MOMENT, Moment};

impl Clock for Moment {
	fn now(&self) -> String {
		String::from(MOMENT)
	}
}
