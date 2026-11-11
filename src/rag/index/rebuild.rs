//! Rebuilding the index
//!
//! One function: it takes the project's root, the paths under `.langdrift/`
//! and the embedder, reads the project's episode records, and gives back a
//! new index with one vector per episode, which it writes.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/microruntimes/episodes/<id>-<slug>.json`
//! - writes `.langdrift/agent/state/rag/index.json`
//!
//! ## Rules
//!
//! - It runs only when the index is missing, was built by another embedder
//!   or is out of step with the records.
//! - Every JSON file in the folder is a record, and its name says the id it
//!   holds, as `<id>-<slug>.json`. One that does not is a stop: an index
//!   built over it could never be found in step (`current`).
//! - No command asks for it: the `reindex` command is removed (spec 0048).
//!
//! Spec 0078.

use crate::files::{layout::Layout, root::Root};
use crate::rag::{
	embedder::{DIMENSIONS, Embedder, ID},
	entry::Entry,
	episode::Episode,
	error::Error,
	index::{Index, VERSION},
};

impl Index {
	pub fn rebuild(
		root: &Root,
		layout: &Layout,
		embedder: &Embedder,
	) -> Result<Self, Error> {
		// A project with no episode folder has no past run: an empty index.
		let records = Episode::recorded(root, layout)?;
		let episodes = records
			.iter()
			.map(|name| {
				let path = layout.episodes.join(name);
				let episode = Episode::load(root, &path)?;

				if Episode::filed(name) != Some(episode.id.as_str()) {
					return Err(Error::Misnamed {
						path,
						id: episode.id,
					});
				}

				Ok(Entry {
					vector: embedder.embed(&episode.intent),
					id: episode.id,
					slug: episode.slug,
				})
			})
			.collect::<Result<Vec<_>, Error>>()?;
		let index = Self {
			version: VERSION,
			embedder: String::from(ID),
			dimensions: DIMENSIONS,
			episodes,
		};

		index.write(root, layout)?;

		Ok(index)
	}
}
