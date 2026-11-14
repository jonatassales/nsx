//! The system of this machine
//!
//! One function: it gives back the system nsx was built for.
//!
//! ## Rules
//!
//! - nsx is built for macOS and Linux only: anything that is not macOS is
//!   treated as Linux.

use crate::local::system::System;

impl System {
	pub fn here() -> Self {
		if cfg!(target_os = "macos") {
			return Self::Mac;
		}

		Self::Linux
	}
}
