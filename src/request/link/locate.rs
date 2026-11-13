//! Finding where a link points
//!
//! One function: it takes the folder the ticket sits in, a link's target as
//! the ticket wrote it and the link's kind, and gives back the link: its file
//! name, its short id and its project-relative path.
//!
//! ## Rules
//!
//! - A target that opens with `/` is read from the project's root. Any other
//!   is read from the ticket's folder.
//! - The path is worked out from the words alone: no file is touched here.
//! - A target that climbs out of the project is refused.
//! - Short ids: a name of the form `<number>-<KIND>-<rest>` is shown as
//!   `<number>-<KIND>`, any other name without its suffix.

use std::path::{Component, Path, PathBuf};

use crate::request::{error::Error, kind::Kind, link::Link};

impl Link {
	pub fn locate(
		folder: &Path,
		target: &str,
		kind: Kind,
	) -> Result<Self, Error> {
		let climbs = || Error::Climbs {
			target: String::from(target),
		};
		let from = match target.strip_prefix('/') {
			Some(rooted) => PathBuf::from(rooted),
			None => folder.join(target),
		};
		let mut path = PathBuf::new();

		for component in from.components() {
			match component {
				Component::CurDir => {}
				Component::Normal(step) => path.push(step),
				Component::ParentDir if path.pop() => {}
				_ => return Err(climbs()),
			}
		}

		let name = path.file_name().ok_or_else(climbs)?.to_string_lossy();
		let stem = name.strip_suffix(kind.suffix).unwrap_or(&name);
		let mut parts = stem.splitn(3, '-');
		let numbered = (parts.next(), parts.next(), parts.next());

		// `0002-PRD-book-a-demo` is shown as `0002-PRD`; `0001-finops` whole.
		let short = match numbered {
			(Some(number), Some(marker), Some(_rest))
				if number.chars().all(|digit| digit.is_ascii_digit())
					&& marker.chars().all(|letter| letter.is_ascii_uppercase()) =>
			{
				format!("{number}-{marker}")
			}
			_ => String::from(stem),
		};

		Ok(Self {
			kind,
			name: name.into_owned(),
			short,
			path,
		})
	}
}
