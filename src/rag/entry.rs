//! One episode in the index
//!
//! What the index keeps of an episode: enough to rank it and to find its
//! record.
//!
//! Definition only: the `Entry` type: the episode's id, its slug and the
//! vector its intent was embedded into.
//!
//! Spec 0078.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
	pub id: String,
	pub slug: String,
	pub vector: Vec<f32>,
}
