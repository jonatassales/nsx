//! The pushed head's short hash
//!
//! One function: it gives back the first seven characters of the pushed
//! head's hash, or nothing while the branch is not pushed.
//!
//! ## Simulation
//!
//! - `pushed`: `✓ pushed     feat/0031-book-a-demo-runtime · 3f9c2e1`
//!
//! ## Rules
//!
//! - The short hash is seven characters, cut from the whole hash here: nsx
//!   never asks `git` for a short one, which grows when seven are ambiguous.
//!
//! Spec 0081.

use crate::delivery::branch::Branch;

const SHORT: usize = 7;

impl Branch {
	pub fn short(&self) -> Option<&str> {
		self.commit.as_deref()?.get(..SHORT)
	}
}
