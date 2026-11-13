//! The title of a project's window
//!
//! One function: it takes the project's root and gives back the title that
//! names its folder.
//!
//! ## Rules
//!
//! - The project's name is the name of the folder that holds `.langdrift/`.
//!   A root with no name of its own, the filesystem's, is named by its path.

use crate::files::root::Root;
use crate::session::title::Title;

impl Title {
	pub fn of(root: &Root) -> Self {
		let project = match root.path.file_name() {
			Some(name) => name.to_string_lossy().into_owned(),
			None => root.path.display().to_string(),
		};

		Self {
			project,
		}
	}
}
