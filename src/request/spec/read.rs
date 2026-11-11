//! Reading the spec
//!
//! One function: it takes the project's root and the spec's link, and gives
//! back the spec with the components it names and its count of acceptance
//! criteria.
//!
//! ## Reads and writes
//!
//! - reads the linked file in the client's artifact folder
//!
//! ## Rules
//!
//! - The `## Components` section has one list item per component, which
//!   starts with the path in backticks, optionally followed by `:<port>`,
//!   bare or in backticks. A port is 1 to 65535.
//! - The `## Acceptance criteria` section has one list item per criterion.
//! - A missing, refused or unreadable link, a section without a list, or an
//!   item that names no path, stops the plan (the stop rule). No link inside
//!   the spec is followed.
//!
//! ## Open
//!
//! - Whether LangDrift imposes these sections on the client's documents is
//!   the owner's decision.
//!
//! Spec 0078.

use crate::files::{
	root::Root,
	text::{DOCUMENT, text},
};
use crate::request::{error::Error, link::Link, named::Named, spec::Spec};

const PORTS: std::ops::RangeInclusive<i64> = 1..=65535;

impl Spec {
	pub fn read(root: &Root, link: &Link) -> Result<Self, Error> {
		// The list items under a `## Heading`, up to the next heading.
		fn items<'text>(text: &'text str, heading: &str) -> Vec<&'text str> {
			let opening = format!("## {heading}");

			text
				.lines()
				.skip_while(|line| line.trim_end() != opening)
				.skip(1)
				.take_while(|line| !line.starts_with('#'))
				.filter_map(|line| line.strip_prefix("- "))
				.map(str::trim)
				.collect()
		}

		// An item that opens with a path in backticks, then an optional port.
		fn named(item: &str) -> Option<Named> {
			let (path, rest) = item.strip_prefix('`')?.split_once('`')?;
			let beside = rest.trim().trim_matches('`');
			let port = match beside.strip_prefix(':') {
				Some(digits) => Some(digits.parse::<i64>().ok()?),
				None => None,
			};
			let usable = port.is_none_or(|port| PORTS.contains(&port));

			usable.then(|| Named {
				path: String::from(path),
				port,
			})
		}

		let written = text(root, &link.path, DOCUMENT)?;
		let listed = |heading: &'static str| {
			let found = items(&written, heading);

			if found.is_empty() {
				return Err(Error::Section {
					path: link.path.clone(),
					section: heading,
				});
			}

			Ok(found)
		};

		let components = listed("Components")?
			.into_iter()
			.map(|item| {
				named(item).ok_or_else(|| Error::Component {
					path: link.path.clone(),
					item: String::from(item),
				})
			})
			.collect::<Result<Vec<_>, _>>()?;
		let criteria = listed("Acceptance criteria")?.len() as u64;

		Ok(Self {
			link: link.clone(),
			components,
			criteria,
		})
	}
}
