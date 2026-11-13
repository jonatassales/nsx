//! A configuration file's code, line by line
//!
//! One function: it takes the text of a `nats.conf` or a `redis.conf` and
//! gives back each line without its comment, with the depth of the blocks
//! the line starts in.
//!
//! ## Rules
//!
//! - A comment runs from `#` to the end of the line, and in `nats.conf` from
//!   `//` at the start of a word as well. Inside a quoted value neither
//!   starts one: a password may hold `#`. A `//` inside a word, as in the
//!   URL `nats://a:6222`, starts none.
//! - In `nats.conf` a block opens with `{`, `[` or `(` and closes with the
//!   matching mark, on one line or over several, outside quotes. A file
//!   wrapped whole in braces, as JSON is, keeps its keys at the top level.
//! - `redis.conf` has no blocks: every line is at the top level.
//! - A quoted value never runs past its line.

use crate::components::manifest::Manifest;

impl Manifest {
	pub fn code(&self, written: &str) -> Vec<(usize, String)> {
		let nats = matches!(self, Self::NatsConf);
		let mut depth = 0_usize;
		let mut lines = Vec::new();

		for line in written.lines() {
			let start = depth;
			let mut code = String::new();
			let mut quote = None;
			let mut letters = line.chars().peekable();
			let mut previous = None;

			while let Some(letter) = letters.next() {
				let word_start =
					!previous.is_some_and(|before: char| !before.is_whitespace());

				previous = Some(letter);

				if let Some(open) = quote {
					let closes = letter == open;

					code.push(letter);
					quote = if closes { None } else { quote };

					continue;
				}

				let slashes =
					nats && word_start && letter == '/' && letters.peek() == Some(&'/');
				let comment = letter == '#' || slashes;

				if comment {
					break;
				}

				let opens = nats && matches!(letter, '{' | '[' | '(');
				let closes = nats && matches!(letter, '}' | ']' | ')');

				if matches!(letter, '"' | '\'') {
					quote = Some(letter);
				}

				if opens {
					depth += 1;
				}

				if closes {
					depth = depth.saturating_sub(1);
				}

				code.push(letter);
			}

			lines.push((start, code));
		}

		// A file wrapped whole in braces keeps its keys at the top level.
		let first = lines
			.iter()
			.find(|(_depth, code)| !code.trim().is_empty())
			.map(|(_depth, code)| code.trim_start().starts_with('{'));

		if first == Some(true) {
			for (depth, code) in lines.iter_mut() {
				let opening = *depth == 0 && code.trim_start().starts_with('{');

				*depth = depth.saturating_sub(1);

				if opening {
					*code = String::from(code.trim_start().trim_start_matches('{'));
				}
			}
		}

		lines
	}
}
