//! A ticket of its own for each end-to-end run
//!
//! Each run plans a ticket no run planned before, so that the console's
//! runtimes are told apart: a four-digit number and a slug that starts with
//! `e2e-`, made of two generated words.

use fake::{Fake, faker::lorem::en::Word};

pub struct Generated {
	pub number: String,
	pub slug: String,
}

impl Generated {
	pub fn id(&self) -> String {
		format!("{}-TICKET-{}", self.number, self.slug)
	}
}

pub fn generated() -> Generated {
	let word = || {
		let made: String = Word().fake();

		made
			.to_lowercase()
			.chars()
			.filter(char::is_ascii_lowercase)
			.collect::<String>()
	};
	let number: u16 = (1000..10000).fake();

	Generated {
		number: number.to_string(),
		slug: format!("e2e-{}-{}-runtime", word(), word()),
	}
}
