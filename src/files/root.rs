//! The project's root
//!
//! The folder that holds `.langdrift/`. Every path nsx reads or writes starts
//! from it, and its name is the project's name in the window title.
//!
//! Definition only: the `Root` type: the path of that folder. Its function is
//! the file in `root/`.
//!
//! ## Rules
//!
//! - The presence of `.langdrift/` is what marks a folder as the root.
//! - `nsx init` does not search for it: its root is the working directory.
//!
//! Shared: used by every folder that reads or writes a file of the project.

pub mod find;

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Root {
	pub path: PathBuf,
}
