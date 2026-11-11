//! The limits on the Plan card
//!
//! One function: it puts the limits statement into the sizing of the plan's
//! cards: of the limits `runtime.md` has no field for, none is requested.
//!
//! ## Simulation
//!
//! - Card: Plan · GPU · DISK, `not runtime.md fields`
//!
//! ## Rules
//!
//! - The list of requested limits is empty, and it is there: empty says
//!   "none requested", absent says "not stated yet".
//! - This function writes no file: the one writer in `handoff/` does.
//!
//! Spec 0079.

use crate::contract::cards::Cards;
use crate::runtime::limits::Limits;

impl Limits {
	pub fn card(&self, cards: &mut Cards) {
		cards.sizing().limits_requested = Some(Vec::new());
	}
}
