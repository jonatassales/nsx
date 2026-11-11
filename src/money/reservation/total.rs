//! Computing the reservation
//!
//! One function: it takes the estimate and gives back the amount to reserve:
//! the estimate plus the 20% margin, in whole cents, or nothing when the
//! amount is too large to state.
//!
//! ## Rules
//!
//! - The result rounds up to the next cent: an estimate of 26 cents reserves
//!   32.
//!
//! Spec 0080.

use crate::money::{cents::Cents, reservation::Reservation, spend::MARGIN};

impl Reservation {
	pub fn total(estimate: Cents) -> Option<Self> {
		let held =
			(i128::from(estimate.0) * i128::from(100 + MARGIN) + 99).div_euclid(100);

		i64::try_from(held).ok().map(|cents| Self {
			amount: Cents(cents),
		})
	}
}
