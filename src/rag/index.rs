//! The index of episodes
//!
//! What the plan searches. It holds one entry per episode of the project, with
//! the vector computed once, when the episode was written, so that
//! similarities do not drift between plans. It lives at
//! `.langdrift/agent/state/rag/index.json`.
//!
//! Definition only: the `Index` type: a version, the id and the dimensions of
//! the embedder that built it, and one `Entry` per episode. Its functions are
//! the files in `index/`.
//!
//! ## Rules
//!
//! - It is rebuilt from the episode records only when it is missing or was
//!   built by another embedder.
//! - No command rebuilds it: it is always right without one.

pub mod current;
pub mod load;
pub mod rebuild;
pub mod search;
pub mod write;

use serde::{Deserialize, Serialize};

use crate::rag::entry::Entry;

pub const VERSION: u64 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Index {
	pub version: u64,
	pub embedder: String,
	pub dimensions: usize,
	pub episodes: Vec<Entry>,
}
