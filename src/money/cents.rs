//! Money, in whole cents
//!
//! Every amount nsx computes, compares or records: an estimate, a budget, a
//! reservation, an observed spend, a release. An amount stays a whole number of
//! cents until a line prints it.
//!
//! Definition only: the `Cents` type: a whole number of US cents. Its
//! functions are the files in `cents/`.
//!
//! ## Rules
//!
//! - Amounts are whole cents.
//! - Rounding belongs to the computation, not to this type: a spend rounds a
//!   half up, a reservation rounds up.
//! - An unknown amount is `null`, never zero cents.
//!
//! ## Open
//!
//! - Whether an amount may be negative. A release is the reservation minus the
//!   observed spend and is not clamped at zero, and what it reads when the
//!   observed spend is the higher of the two is not settled.

pub mod format;
pub mod parse;

use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Cents(pub i64);
