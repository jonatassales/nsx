//! Refusing a path that leaves the project
//!
//! One function: it takes the project's root and a path a document names, and
//! gives back that path inside the project, or refuses it.
//!
//! ## Rules
//!
//! - A path that leaves the project is refused: an absolute path, or one
//!   that climbs with `..`.
//! - A path that is a symbolic link is refused.
//! - A refusal is a stop: a ticket's link that leaves the project or is a
//!   symlink stops the plan before its next line (spec 0053).
//!
//! ## Open
//!
//! - Spec 0053 also refuses a link that points at a secret. No spec says how
//!   a secret is recognised, or whether that check lives here or in `request`.
//!
//! Shared: used by `request` and `components`.

use std::path::{Component, Path, PathBuf};

use crate::files::{error::Error, root::Root};

pub fn within(root: &Root, named: &Path) -> Result<PathBuf, Error> {
	let mut inside = root.path.clone();

	for component in named.components() {
		let Component::Normal(step) = component else {
			if component == Component::CurDir {
				continue;
			}

			return Err(Error::Outside {
				path: named.to_path_buf(),
			});
		};

		inside.push(step);

		let linked = std::fs::symlink_metadata(&inside)
			.is_ok_and(|found| found.file_type().is_symlink());

		if linked {
			return Err(Error::Linked {
				path: named.to_path_buf(),
			});
		}
	}

	Ok(inside)
}
