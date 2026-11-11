//! The sized plan and its `runtime.md` draft
//!
//! Once the components are resolved, the plan sizes them inside the bound: it
//! sums their proposed shares, checks the sum against the bound and computes
//! the headroom. It states what `runtime.md` does not carry, validates the
//! draft against the schema, counts the errors and writes the draft with its
//! revision. What `nsx agent run` later applies is what this draft records.
//!
//! - `Sizing`: the summed shares, the bound and the headroom
//! - `Limits`: what `runtime.md` does not carry
//! - `Revision`: the revision of a plan
//! - `Draft`: the `runtime.md` draft
//!
//! ## Rules
//!
//! - The plan's order after `crossCheck`: `size`, then `limits`, then `price`
//!   (spec 0056), then `validate`, then `draft` (spec 0050). The `price` and
//!   `draft` lines are not printed from this folder.
//! - Sizing each component belongs to spec 0066, the bound to spec 0054, the
//!   suggestion to spec 0051, and the fence's fields and schema to spec 0049.
//! - Handing the draft to the team, and saying it is proposed or pending
//!   authorization, belong to spec 0050.
//!
//! Spec 0079.

pub mod draft;
pub mod error;
pub mod limits;
pub mod revision;
pub mod sizing;
