//! Whether a ledger is its folder's
//!
//! One function: it takes a ledger, the project's root, the name of the
//! folder it was read from and the runtime asked about, and says whether
//! nsx may clean up from it.
//!
//! ## Rules
//!
//! - The ledger names the run of its folder and the runtime asked about.
//! - Its space has the shape nsx gives that run's space: the run's id, cut,
//!   then eight hexadecimal characters. The digest itself is not compared,
//!   so a project that was moved or renamed still cleans up what it left
//!   under its old folder.
//! - Any other space is not one nsx made for this run: a ledger that was
//!   edited to name another namespace is never cleaned up from.
//!
//! Spec 0081.

use crate::files::root::Root;
use crate::local::execution::Execution;

const DIGITS: usize = 8;

impl Execution {
	pub fn placed(&self, root: &Root, folder: &str, runtime: &str) -> bool {
		let given = Self::space(root, &self.run);
		let named = given.rsplit_once('-').map(|(stem, _mark)| stem);
		let kept = self.space.rsplit_once('-');
		let marked = kept.is_some_and(|(_stem, mark)| {
			mark.len() == DIGITS
				&& mark
					.chars()
					.all(|digit| digit.is_ascii_digit() || ('a'..='f').contains(&digit))
		});
		let shaped = marked && kept.map(|(stem, _mark)| stem) == named;

		self.run == folder && self.runtime == runtime && shaped
	}
}
