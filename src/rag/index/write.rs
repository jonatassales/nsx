//! Writing the index
//!
//! One function: it takes the project's root and the paths under
//! `.langdrift/`, and writes the index whole.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/agent/state/rag/index.json`
//!
//! Spec 0078.

use serde_json::to_vec;

use crate::files::{inside::inside, layout::Layout, root::Root, write::write};
use crate::rag::{error::Error, index::Index};

impl Index {
	pub fn write(&self, root: &Root, layout: &Layout) -> Result<(), Error> {
		let file = inside(root, &layout.index)?;
		let written = to_vec(self).map_err(|cause| Error::Unwritten {
			path: file.clone(),
			cause,
		})?;

		Ok(write(&file, &written)?)
	}
}
