//! Whether the index is in step with the records
//!
//! One function: it takes the index, the project's root and the paths under
//! `.langdrift/`, and says whether the index lists exactly the episodes
//! recorded on the disk.
//!
//! ## Reads and writes
//!
//! - lists `.langdrift/microruntimes/episodes/`, for the records' ids
//!
//! ## Rules
//!
//! - A record with no entry, an entry with no record, or an entry listed
//!   twice means the index is out of step. A run writes its episode's record
//!   before the index, so an index whose write failed is found here by the
//!   next plan.
//! - Only the ids are compared, as the records' file names give them
//!   (`filed`): the records are not read. A rebuild refuses a record whose
//!   name does not say the id it holds, so the two always agree.
//! - A JSON file in the folder whose name is no record's puts the index out
//!   of step: the rebuild then says what is wrong with it.
//!
//! Spec 0078.

use crate::files::{layout::Layout, root::Root};
use crate::rag::{episode::Episode, error::Error, index::Index};

impl Index {
	pub fn current(&self, root: &Root, layout: &Layout) -> Result<bool, Error> {
		let names = Episode::recorded(root, layout)?;
		let recorded = names
			.iter()
			.map(|name| Episode::filed(name))
			.collect::<Option<Vec<_>>>();

		let Some(mut recorded) = recorded else {
			return Ok(false);
		};

		let mut indexed = self
			.episodes
			.iter()
			.map(|entry| entry.id.as_str())
			.collect::<Vec<_>>();

		recorded.sort_unstable();
		indexed.sort_unstable();

		Ok(indexed == recorded)
	}
}
