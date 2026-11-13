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
//! - The plan's order after `crossCheck`: `size`, then `limits`, then
//!   `price`, then `validate`, then `draft`. The `price` and `draft` lines are
//!   not printed from this folder: `money/` and `handoff/` print them.
//! - Sizing each component belongs to `components/`, the bound to `harness/`,
//!   the suggestion to `judgment/`, and the fence's fields and schema to
//!   `contract/`.
//! - Handing the draft to the team, and saying it is proposed or pending
//!   authorization, belong to `handoff/`.

pub mod draft;
pub mod error;
pub mod limits;
pub mod revision;
pub mod sizing;
