//! A folder's failure, as a stop
//!
//! One function: it takes the failure of any folder and gives back the stop
//! that carries its sentence. Every folder words its own failures, each as
//! one plain sentence, so the stop adds nothing to them.
//!
//! ## Rules
//!
//! - The sentence is the failure's own, unchanged.

use crate::session::stop::Stop;

impl<Failure: std::error::Error> From<Failure> for Stop {
	fn from(failure: Failure) -> Self {
		Self {
			sentence: failure.to_string(),
		}
	}
}
