//! Reading one line of standard input
//!
//! One function: it reads one line the person entered and gives it back
//! without its line break, or nothing when the input has ended.
//!
//! ## Reads and writes
//!
//! - reads one line from standard input
//!
//! ## Rules
//!
//! - The answer is echoed as typed: it is one word and no secret.
//!
//! Spec 0080.

use crate::flow::{keyboard::Keyboard, reader::Reader};

impl Reader for Keyboard {
	fn line(&self) -> std::io::Result<Option<String>> {
		let mut entered = String::new();
		let read = std::io::stdin().read_line(&mut entered)?;

		if read == 0 {
			return Ok(None);
		}

		Ok(Some(String::from(entered.trim_end_matches(['\r', '\n']))))
	}
}
