//! The port a configuration file states
//!
//! One function: it takes the text of a `nats.conf` or a `redis.conf` and
//! gives back the value its top-level port key holds, as written, if the
//! file has one.
//!
//! ## Rules
//!
//! - The key is `port`, in any case, written `port 4222`, `port: 4222` or
//!   `port = 4222`, with or without quotes around the key or the value and
//!   with or without a `,` or a `;` after it.
//! - In `nats.conf` the key may also be `listen`, in any case, written
//!   `listen: 4222` or `listen: 0.0.0.0:4222`: the value is what follows the
//!   last colon.
//! - Comments and blocks are read as `code.rs` says. Only a key at the top
//!   level counts: a `port` inside a block is another listener's.
//! - The value is given back as written. Whether it is a port is
//!   `port.rs`'s to say.
//!
//! Spec 0079.

use crate::components::manifest::Manifest;

impl Manifest {
	pub fn keyed(&self, written: &str) -> Option<String> {
		let nats = matches!(self, Self::NatsConf);

		for (depth, code) in self.code(written) {
			let stated = code.trim();
			let end = stated.find([' ', '\t', ':', '=']).unwrap_or(stated.len());
			let (key, rest) = stated.split_at(end);
			let key = key.trim_matches(['"', '\'']);
			let value = rest
				.trim_start_matches([' ', '\t', ':', '='])
				.trim_end_matches([' ', '\t', ',', ';', '}'])
				.trim_matches(['"', '\'']);
			let listens = nats && key.eq_ignore_ascii_case("listen");
			let top_level_port =
				depth == 0 && (listens || key.eq_ignore_ascii_case("port"));
			let port = if listens {
				value.rsplit(':').next().unwrap_or(value)
			} else {
				value
			};

			if top_level_port {
				return Some(String::from(port));
			}
		}

		None
	}
}
