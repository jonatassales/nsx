//! Loading the driver configuration
//!
//! One function: it takes the project's root and the paths under
//! `.langdrift/`, reads the project's `driver.json` and gives back the
//! configuration, or the reason it is refused.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/harness/driver.json`
//!
//! ## Rules
//!
//! - It runs once the gate has opened, before the first stage waits: a run
//!   over its budget still pauses without a configuration (owner, plan of
//!   2026-11-11).
//! - A missing file, a field the configuration does not define, or `args`
//!   without `{{model}}` and `{{effort}}`, is refused. The run then stops
//!   before its first stage, with nothing paid (the stop rule).

use crate::contract::decode::decode;
use crate::driver::{config::Config, error::Error};
use crate::files::{
	inside::inside,
	layout::Layout,
	root::Root,
	text::{DOCUMENT, text},
};

impl Config {
	pub fn load(root: &Root, layout: &Layout) -> Result<Self, Error> {
		let file = inside(root, &layout.driver)?;

		if !file.exists() {
			return Err(Error::Missing {
				path: file,
			});
		}

		let written = text(root, &layout.driver, DOCUMENT)?;
		let config: Self = decode(&written).map_err(|cause| Error::Config {
			path: file.clone(),
			cause,
		})?;
		let carries = |placeholder: &str| {
			config.args.iter().any(|arg| arg.contains(placeholder))
		};
		let chosen = carries("{{model}}") && carries("{{effort}}");

		if !chosen {
			return Err(Error::Arguments {
				path: file,
			});
		}

		Ok(config)
	}
}
