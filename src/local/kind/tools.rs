//! Where nsx keeps its tools
//!
//! One function: it gives back the project's tools folder,
//! `.langdrift/agent/state/tools/`.
//!
//! ## Rules
//!
//! - The folder is inside the project's `.langdrift/`: a path that leaves it
//!   through a symbolic link is refused.

use std::path::PathBuf;

use crate::files::{inside::inside, layout::Layout, root::Root};
use crate::local::{error::Error, kind::Kind};

impl Kind<'_> {
	pub fn tools(&self) -> Result<PathBuf, Error> {
		let root = Root::find(&self.directory)?;
		let folder = inside(&root, &Layout::under(&root).tools)?;

		Ok(folder)
	}
}
