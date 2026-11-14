//! Generating a ticket: a number and two words, with the `fake` crate

use fake::{Fake, faker::lorem::en::Word};

use crate::e2e::generated::Generated;

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
