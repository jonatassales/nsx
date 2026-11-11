//! The port a `package.json` start script states
//!
//! One function: it takes the text of a `package.json` and gives back the
//! value its `start` script gives the port, as written, if it gives one.
//!
//! ## Rules
//!
//! - Only the `start` script is read: it is the one the component runs.
//!   Another script's port, such as `dev`'s, is not the component's.
//! - `PORT=8081`, `--port 8081` and `--port=8081` state the port whatever
//!   follows them: the value is given back as written, without its quotes
//!   or a closing `;`, and `port.rs` stops the plan when it is not a port.
//! - `-p 8081` and `-p8081` state it only with digits: a script is free
//!   text, and `-p` followed by anything else is another tool's flag, as in
//!   `tsc -p tsconfig.json`.
//! - A `package.json` that is not JSON is refused.
//!
//! ## Open
//!
//! - `-p` followed by digits is read as the port wherever it stands, so
//!   `mkdir -p 2026 && node server.js` reads 2026. Telling the server's flag
//!   from another tool's needs to know which command starts the server.
//!
//! Spec 0079.

use serde_json::Value;

use crate::components::manifest::Manifest;

impl Manifest {
	pub fn scripted(written: &str) -> Result<Option<String>, serde_json::Error> {
		let package: Value = serde_json::from_str(written)?;
		let start = package
			.get("scripts")
			.and_then(|scripts| scripts.get("start"))
			.and_then(Value::as_str);
		let Some(script) = start else {
			return Ok(None);
		};
		let digits = |value: &&str| {
			!value.is_empty() && value.chars().all(|digit| digit.is_ascii_digit())
		};
		let mut words = script.split_whitespace();

		while let Some(word) = words.next() {
			let stated = match word {
				"--port" => words.next(),
				"-p" => words.next().filter(digits),
				_ => word
					.strip_prefix("PORT=")
					.or_else(|| word.strip_prefix("--port="))
					.or_else(|| word.strip_prefix("-p").filter(digits)),
			};
			let value = stated.map(|value| {
				value
					.trim_end_matches([';', '&', '|'])
					.trim_matches(['"', '\''])
			});

			if let Some(value) = value {
				return Ok(Some(String::from(value)));
			}
		}

		Ok(None)
	}
}
