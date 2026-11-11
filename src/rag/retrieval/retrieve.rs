//! Retrieving the episodes for a ticket
//!
//! One function: it takes the project's root, the paths under `.langdrift/`
//! and the ticket's intent, its title and body, and gives back the
//! retrieval. It embeds the intent into one query vector, takes the
//! six most similar episodes from the index, applies the threshold and counts
//! the kept, the dropped and the cited ones that measured resources.
//!
//! ## Rules
//!
//! - One ticket is one query vector.
//! - It makes no paid call.
//! - It does not decide a candidate's reason: the judge does (spec 0051).
//!
//! Spec 0078.

use crate::files::{layout::Layout, root::Root};
use crate::rag::{
	embedder::Embedder, episode::Episode, error::Error, index::Index,
	retrieval::Retrieval, threshold::THRESHOLD,
};

impl Retrieval {
	pub fn retrieve(
		root: &Root,
		layout: &Layout,
		intent: &str,
	) -> Result<Self, Error> {
		let embedder = Embedder;
		let query = embedder.embed(intent);
		let index = match Index::load(root, layout)? {
			Some(index) => index,
			None => Index::rebuild(root, layout, &embedder)?,
		};
		let candidates = index.search(&query);

		// The judge reads each candidate's record, and a cited one says
		// whether its run measured resources.
		let records = candidates
			.iter()
			.map(|candidate| {
				let record = format!("{}-{}.json", candidate.id, candidate.slug);

				Episode::load(root, &layout.episodes.join(record))
			})
			.collect::<Result<Vec<_>, _>>()?;
		let cited_measured = candidates
			.iter()
			.zip(&records)
			.filter(|(candidate, record)| candidate.kept && record.measured.is_some())
			.count() as u64;
		let total = candidates.len() as u64;
		let kept =
			candidates.iter().filter(|candidate| candidate.kept).count() as u64;

		Ok(Self {
			threshold: THRESHOLD,
			candidates,
			records,
			kept,
			dropped: total - kept,
			total,
			cited: kept,
			cited_measured,
		})
	}
}
