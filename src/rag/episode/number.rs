//! The number in an episode's id
//!
//! One function: it takes an id and gives back the episode's number, when
//! the id is an episode's: `ep-<digits>`.
//!
//! ## Rules
//!
//! - The number has four digits or more, and nothing but digits.
//! - Any other text is no episode's id.
//!
//! Spec 0078.

use crate::rag::episode::Episode;

impl Episode {
	pub fn number(id: &str) -> Option<u32> {
		let digits = id.strip_prefix("ep-")?;
		let numbered =
			digits.len() >= 4 && digits.chars().all(|digit| digit.is_ascii_digit());

		if !numbered {
			return None;
		}

		digits.parse().ok()
	}
}
