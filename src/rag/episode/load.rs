//! Loading an episode
//!
//! One function: it takes the project's root and a record's path, reads the
//! record and gives back the episode. The
//! judge reads a candidate's components, resource and intent from it, and the
//! index is rebuilt from these records.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/microruntimes/episodes/<id>-<slug>.json`

use std::path::Path;

use crate::contract::decode::decode;
use crate::files::{
	root::Root,
	text::{DOCUMENT, text},
};
use crate::rag::{episode::Episode, error::Error};

impl Episode {
	pub fn load(root: &Root, path: &Path) -> Result<Self, Error> {
		let written = text(root, path, DOCUMENT)?;

		decode(&written).map_err(|cause| Error::Episode {
			path: path.to_path_buf(),
			cause,
		})
	}
}
