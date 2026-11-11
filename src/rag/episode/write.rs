//! Writing a run's episode
//!
//! One function: it takes a finished run's episode, with its ticket, its
//! intent, its components and its suggested resource, and gives back the
//! episode it wrote. It gives the episode the next free `ep-NNNN` number,
//! embeds its intent once and adds the vector to the index at the same
//! moment.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/microruntimes/episodes/`, for the highest number
//! - writes `.langdrift/microruntimes/episodes/<id>-<slug>.json`
//! - writes `.langdrift/agent/state/rag/index.json`
//!
//! ## Rules
//!
//! - It runs once per finished run, after the `ready` line (spec 0063), once
//!   the run's outcome is `completed` (spec 0065).
//! - A run paused at the gate leaves no episode (spec 0056).
//! - The number is the highest existing number plus one, whatever id the
//!   episode came with, and it keeps all its digits past 9999.
//! - The slug is a lowercase label, so the record's file is always inside
//!   the episodes folder.
//! - The record is written first. A plan that finds the index out of step
//!   with the records builds it again (`index/current.rs`), so an index that
//!   could not be written after the record costs nothing.
//! - A project whose index does not exist yet, or was built by another
//!   embedder, gets one built from its episodes, the new one among them.
//!   Otherwise the index gains the one entry, without a rebuild.
//! - It prints nothing.
//!
//! Spec 0078.

use crate::contract::encode::encode;
use crate::contract::label::label;
use crate::files::{inside::inside, layout::Layout, root::Root, write::write};
use crate::rag::{
	embedder::Embedder, entry::Entry, episode::Episode, error::Error,
	index::Index,
};

impl Episode {
	pub fn write(
		mut self,
		root: &Root,
		layout: &Layout,
		embedder: &Embedder,
	) -> Result<Self, Error> {
		if !label(&self.slug) {
			return Err(Error::Slug {
				slug: self.slug,
			});
		}

		// The highest number a record's name holds: none yet counts as zero.
		let highest = Self::recorded(root, layout)?
			.iter()
			.filter_map(|name| Self::filed(name))
			.filter_map(Self::number)
			.max()
			.unwrap_or(0);

		// The index is read before the record is written: one built now would
		// already hold the new episode.
		let known = Index::load(root, layout)?;
		let next = highest.checked_add(1).ok_or(Error::Exhausted)?;

		self.id = format!("ep-{next:04}");

		let name = format!("{}-{}.json", self.id, self.slug);
		let file = inside(root, &layout.episodes.join(name))?;
		let mut written = encode(&self).map_err(|cause| Error::Unwritten {
			path: file.clone(),
			cause,
		})?;

		written.push('\n');
		write(&file, written.as_bytes())?;

		let Some(mut index) = known else {
			Index::rebuild(root, layout, embedder)?;

			return Ok(self);
		};

		index.episodes.push(Entry {
			id: self.id.clone(),
			slug: self.slug.clone(),
			vector: embedder.embed(&self.intent),
		});
		index.write(root, layout)?;

		Ok(self)
	}
}
