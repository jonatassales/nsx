//! The run's closing comment
//!
//! One function: it takes the ending and the pull request's number, and gives
//! back the last line of the run, which says what the owner does next.
//!
//! ## Simulation
//!
//! - `next`: `# next: the owner reviews #19 and decides whether to merge`
//! - `next`, with Review unticked: `# next: the merge stays the owner's`
//!
//! ## Rules
//!
//! - It follows the `ready` line (spec 0063), which supplies the pull
//!   request's number.
//! - A run paused at the gate ends without it.
//!
//! Spec 0080.

use crate::flow::ending::Ending;
use crate::session::line::Line;

impl Ending {
	pub fn line(&self, pull: u64) -> Line {
		let text = match self {
			Self::Review => {
				format!("next: the owner reviews #{pull} and decides whether to merge")
			}
			Self::Merge => String::from("next: the merge stays the owner's"),
		};

		Line::comment("next", &text)
	}
}
