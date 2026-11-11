//! A path of the project, checked
//!
//! Where every read and write of the project's own files gets its path from.
//!
//! One function: it takes the project's root and a path, and gives back the
//! path inside the project, or refuses it.
//!
//! ## Rules
//!
//! - The path is read from the root. A path given whole, as `Layout` gives
//!   them, must sit under the root.
//! - Either way it never leaves the project and never passes through a
//!   symbolic link: `within` decides.
//!
//! Spec 0077.

use std::path::{Path, PathBuf};

use crate::files::{error::Error, root::Root, within::within};

pub fn inside(root: &Root, path: &Path) -> Result<PathBuf, Error> {
	let named = path.strip_prefix(&root.path).unwrap_or(path);

	within(root, named)
}
