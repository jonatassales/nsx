//! Loading the index
//!
//! One function: it takes the project's root and the paths under
//! `.langdrift/`, reads the index file and gives back the index, or nothing
//! when there is none to search.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/agent/state/rag/index.json`
//!
//! ## Rules
//!
//! - A missing index, or one built by another embedder, is not searched: it is
//!   rebuilt from the episode records first (`rebuild`).
//! - Neither is an index out of step with the episode records on the disk
//!   (`current`): it is built again.
//! - An index of this embedder whose vector is not the embedder's width is
//!   refused: a similarity is never worked out from a vector that cannot
//!   have one.
//! - The index is read up to 256 MiB, which is about sixty thousand episodes.
//!
//! Spec 0078.

use std::io::ErrorKind;

use crate::contract::decode::decode;
use crate::files::error::Error as Unread;
use crate::files::{layout::Layout, root::Root, text::text};
use crate::rag::{
	embedder::{DIMENSIONS, ID},
	error::Error,
	index::Index,
};

// The index grows with every run: about four kilobytes an episode.
const LIMIT: u64 = 1 << 28;

impl Index {
	pub fn load(root: &Root, layout: &Layout) -> Result<Option<Self>, Error> {
		let written = match text(root, &layout.index, LIMIT) {
			Ok(written) => written,
			Err(Unread::Unreadable {
				cause,
				..
			}) if cause.kind() == ErrorKind::NotFound => return Ok(None),
			Err(refusal) => return Err(Error::File(refusal)),
		};
		let index = decode::<Self>(&written).map_err(|cause| Error::Index {
			path: layout.index.clone(),
			cause,
		})?;
		let ours = index.embedder == ID && index.dimensions == DIMENSIONS;

		if !ours {
			return Ok(None);
		}

		// An index of this embedder holds vectors of its width, and no other.
		let narrow = index
			.episodes
			.iter()
			.find(|entry| entry.vector.len() != DIMENSIONS);

		if let Some(entry) = narrow {
			return Err(Error::Width {
				path: layout.index.clone(),
				id: entry.id.clone(),
			});
		}

		if !index.current(root, layout)? {
			return Ok(None);
		}

		Ok(Some(index))
	}
}
