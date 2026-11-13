//! Reading the machine's clock
//!
//! One function: it gives back the current moment, to the second, as an
//! RFC 3339 timestamp in UTC.

use jiff::Timestamp;

use crate::evidence::{clock::Clock, watch::Watch};

impl Clock for Watch {
	fn now(&self) -> String {
		// Whole seconds: a recorded moment claims no more than it needs.
		Timestamp::now().strftime("%Y-%m-%dT%H:%M:%SZ").to_string()
	}
}
