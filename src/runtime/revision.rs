//! The revision of a plan
//!
//! Every draft of a runtime has a revision, so that the run can name the one
//! it applies. The first plan of a runtime is revision 1. A later plan of the
//! same runtime is the previous revision plus one.
//!
//! Definition only: the `Revision` type: a whole number, 1 or more. Its
//! function is the file in `revision/`.
//!
//! ## Rules
//!
//! - Planning the same ticket again writes the next revision over the draft.
//! - No lock refuses a new plan while an earlier run of the runtime exists.
//! - The simulation shows only revision 1. Nothing about a later revision
//!   prints differently.

pub mod next;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Revision(pub i64);
