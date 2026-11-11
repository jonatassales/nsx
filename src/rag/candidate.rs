//! An episode candidate
//!
//! One row of the Analysis card's EPISODES section: an episode the search
//! found, how similar it is to the ticket's intent, and whether the plan keeps
//! it.
//!
//! Definition only: the `Candidate` type: the episode's id and slug, its
//! similarity at two decimals, its verdict, kept or dropped, and its reason,
//! unknown until the judge gives it.
//!
//! ## Simulation
//!
//! - Card: Analysis · EPISODES rows: the id, the slug, the similarity with its
//!   bar, then the verdict, as in `ep-0412 checkout-preview-sizing 0.81 kept`
//!   and `ep-0297 docs-search-index 0.52 dropped · docs only`
//!
//! ## Rules
//!
//! - The verdict is the threshold's. The reason is the judge's, from its
//!   closed set (spec 0051).
//!
//! Spec 0078.

#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
	pub id: String,
	pub slug: String,
	pub similarity: f64,
	pub kept: bool,
	pub reason: Option<String>,
}
