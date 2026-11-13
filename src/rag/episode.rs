//! The episode
//!
//! One past run, as the next plan finds it. Every finished run leaves exactly
//! one episode with its own id, so the project remembers every run and not
//! only the last of each runtime. It lives at
//! `.langdrift/microruntimes/episodes/<id>-<slug>.json`.
//!
//! Definition only: the `Episode` type: its `ep-NNNN` id, the slug and the id
//! of the ticket it delivered, the intent it is embedded from, its components,
//! each a `Part`, its `Resource` and what it `Measured`. Its functions are
//! the files in `episode/`.
//!
//! ## Rules
//!
//! - The intent is the delivered ticket's title and body.
//! - The measured peaks are `null` when the run measured neither cpu nor
//!   memory: unknown is never zero. Runs measure neither today, so every
//!   episode nsx writes has `measured: null`.
//! - An episode with no components is one whose run changed no runtime.

pub mod filed;
pub mod load;
pub mod number;
pub mod recorded;
pub mod write;

use serde::{Deserialize, Serialize};

use crate::rag::{measured::Measured, part::Part, resource::Resource};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Episode {
	pub id: String,
	pub slug: String,
	pub ticket: String,
	pub intent: String,
	pub components: Vec<Part>,
	pub resource: Resource,
	#[serde(deserialize_with = "Option::deserialize")]
	pub measured: Option<Measured>,
}
