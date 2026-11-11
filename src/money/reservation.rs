//! The reservation
//!
//! The part of the budget a run holds before it spends: the estimate plus a
//! fixed 20% margin. The gate compares it with the budget. When the gate
//! opens, the run holds it, and what the run does not spend is released at the
//! end.
//!
//! Definition only: the `Reservation` type: the reserved amount in cents, made
//! from the estimate and the fixed margin. Its functions are the files in
//! `reservation/`.
//!
//! ## Rules
//!
//! - The margin is fixed at 20%. It is not asked at `nsx init` and not read
//!   from an input.
//! - Nothing is reserved before the gate opens, and planning reserves nothing.
//! - Amounts are whole cents.
//!
//! ## Open
//!
//! - Whether a team may change the margin, and where. The simulation never
//!   says who sets it.
//!
//! Spec 0080.

pub mod total;

use crate::money::cents::Cents;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reservation {
	pub amount: Cents,
}
