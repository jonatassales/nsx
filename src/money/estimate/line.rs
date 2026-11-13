//! The estimate's line
//!
//! One function: it gives back the line a run prints before anything is
//! reserved: the estimate, the expected tokens and the criterion it satisfies.
//!
//! ## Simulation
//!
//! - `estimate`: `console › estimate   $9.50 · 3.5M input · 250K output · list prices · FIN-02`
//!
//! ## Rules
//!
//! - The amount and the tokens are the applied pair's. The line above is
//!   Sonnet at medium effort.
//! - Token counts are compact, as in `3.5M` and `250K`, and exact: thousands
//!   under a million end in `K`, and millions keep every digit they have.
//! - The line cites FIN-02: the estimate comes before any paid call.
//! - The plan's line with the same label, on the runtime's price, belongs to
//!   the price list.

use crate::money::estimate::Estimate;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Estimate {
	pub fn line(&self) -> Line {
		// Thousands of tokens, as the card writes them: 3500 is `3.5M`, and
		// no digit is dropped, so 1050 is `1.05M`.
		let compact = |thousands: u32| {
			if thousands < 1000 {
				return format!("{thousands}K");
			}

			let decimals = format!("{:03}", thousands % 1000);
			let kept = decimals.trim_end_matches('0');

			match kept {
				"" => format!("{}M", thousands / 1000),
				_ => format!("{}.{kept}M", thousands / 1000),
			}
		};

		Line {
			id: String::from("estimate"),
			kind: Kind::Output,
			shape: Shape::Labeled {
				label: String::from("estimate"),
				value: format!(
					"{} · {} input · {} output · list prices · FIN-02",
					self.amount.format(),
					compact(self.tokens.input),
					compact(self.tokens.output)
				),
			},
		}
	}
}
