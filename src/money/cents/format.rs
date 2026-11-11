//! Printing an amount
//!
//! One function: it takes an amount in cents and gives back the text a
//! terminal line shows: US dollars with two decimals, as in `$9.50` and
//! `$11.40`.
//!
//! ## Rules
//!
//! - Every terminal line uses this one format: the simulation's, US English.
//!   Thousands are grouped with a comma, as in `$1,234.56`, and an amount
//!   below zero opens with a minus sign, as in `-$1.60`.
//! - The card formats money in the viewer's locale. That is the console's
//!   concern: the records carry cents, never formatted text.
//!
//! Spec 0079.

use crate::money::cents::Cents;

const GROUP: usize = 3;

impl Cents {
	pub fn format(&self) -> String {
		let sign = if self.0 < 0 { "-" } else { "" };
		let cents = self.0.unsigned_abs();
		let dollars = (cents / 100).to_string();

		// The dollars are grouped in threes from the right, as in `$1,234.56`.
		let grouped = dollars
			.as_bytes()
			.rchunks(GROUP)
			.rev()
			.map(String::from_utf8_lossy)
			.collect::<Vec<_>>()
			.join(",");

		format!("{sign}${grouped}.{:02}", cents % 100)
	}
}
