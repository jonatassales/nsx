//! Embedding a text
//!
//! One function: it takes a text and gives back its vector, of unit length.
//!
//! ## Rules
//!
//! - It is deterministic, and it makes no paid call.
//! - The words are the lowercase runs of letters and digits, kept whole
//!   across a single dot or dash, as in `signup-api` and `Cargo.toml`.
//! - Each word, and each pair of neighbouring words, is hashed with FNV-1a
//!   into one of the dimensions, added or subtracted by the hash's last bit so
//!   that collisions spread out. A count above zero is dampened to
//!   `1 + ln(count)`.
//! - Its figures belong to its id: an index built under that id stays in
//!   step, and changing any figure means a new id.

use crate::rag::embedder::{DIMENSIONS, Embedder};

const OFFSET: u32 = 2_166_136_261;

const PRIME: u32 = 16_777_619;

impl Embedder {
	pub fn embed(&self, text: &str) -> Vec<f32> {
		fn hash(term: &str) -> u32 {
			term.bytes().fold(OFFSET, |hash, byte| {
				(hash ^ u32::from(byte)).wrapping_mul(PRIME)
			})
		}

		let mut words = Vec::<String>::new();
		let mut word = String::new();
		let mut joiner = None;

		// A dot or a dash joins two runs only when a run follows it at once.
		for character in text.to_lowercase().chars().chain([' ']) {
			let plain = character.is_ascii_lowercase() || character.is_ascii_digit();
			let joins =
				matches!(character, '.' | '-') && !word.is_empty() && joiner.is_none();

			if plain {
				word.extend(joiner.take());
				word.push(character);

				continue;
			}

			if joins {
				joiner = Some(character);

				continue;
			}

			joiner = None;

			if !word.is_empty() {
				words.push(std::mem::take(&mut word));
			}
		}

		let mut vector = vec![0.0_f32; DIMENSIONS];
		let pairs = words
			.windows(2)
			.map(|pair| format!("{} {}", pair[0], pair[1]));

		for term in words.iter().cloned().chain(pairs) {
			let hashed = hash(&term);
			let slot = hashed as usize % DIMENSIONS;

			vector[slot] += if hashed & 1 == 0 { 1.0 } else { -1.0 };
		}

		for cell in vector.iter_mut().filter(|cell| **cell > 0.0) {
			*cell = (1.0 + f64::from(*cell).ln()) as f32;
		}

		let squares = vector
			.iter()
			.map(|cell| f64::from(*cell) * f64::from(*cell))
			.sum::<f64>();

		if squares == 0.0 {
			return vector;
		}

		let length = squares.sqrt() as f32;

		vector.iter().map(|cell| cell / length).collect()
	}
}
