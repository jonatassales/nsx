//! Opening the gate
//!
//! One function: it takes the reservation and the budget, and gives back the
//! gate, opened or paused.
//!
//! ## Rules
//!
//! - The gate is decided before any paid call (FIN-02, FIN-03).
//! - A reservation equal to the budget opens the gate.
//!
//! Spec 0080.

use crate::money::{cents::Cents, gate::Gate};

impl Gate {
	pub fn open(reserved: Cents, budget: Cents) -> Self {
		if reserved > budget {
			return Self::Paused {
				reserved,
				budget,
			};
		}

		Self::Opened {
			reserved,
			budget,
		}
	}
}
