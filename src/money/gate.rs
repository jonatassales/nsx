//! The gate
//!
//! Decides whether paid work may start. It compares the reservation with the
//! budget of the run: when the reservation fits, the gate opens; when it
//! exceeds the budget, the gate pauses and the run ends with nothing paid for.
//!
//! Definition only: the `Gate` type and its two outcomes, opened and paused.
//! Its functions are the files in `gate/`.
//!
//! ## Rules
//!
//! - A paused gate is not a failure: the run ends with exit code 0.
//! - Amounts are whole cents.

pub mod line;
pub mod open;

use crate::money::cents::Cents;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gate {
	Opened { reserved: Cents, budget: Cents },
	Paused { reserved: Cents, budget: Cents },
}
