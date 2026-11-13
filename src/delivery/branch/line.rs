//! The pushed line
//!
//! One function: it takes the pushed branch and gives back the line that
//! names the branch and the pushed commit's short hash, or nothing while
//! the branch is not pushed.
//!
//! ## Simulation
//!
//! - `pushed`: `✓ pushed     feat/0031-book-a-demo-runtime · 3f9c2e1`
//!
//! ## Rules
//!
//! - It follows the `completed` line.
//! - It reports what the run did. It always carries the hash and never reads
//!   `unknown`.

use crate::delivery::branch::Branch;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Branch {
	pub fn line(&self) -> Option<Line> {
		let commit = self.short()?;

		Some(Line {
			id: String::from("pushed"),
			kind: Kind::Success,
			shape: Shape::Result {
				label: String::from("pushed"),
				value: format!("{} · {commit}", self.name),
			},
		})
	}
}
