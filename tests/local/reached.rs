//! kind as a test reaches it: in a project, over the runner that starts
//! nothing, with no tool to bring and no web to download from

use std::cell::Cell;

use nsx::files::root::Root;
use nsx::local::kind::Kind;

use crate::simulation::fake::{programs::Programs, unreached::Unreached};

pub fn reached<'run>(programs: &'run Programs, root: &Root) -> Kind<'run> {
	Kind {
		programs,
		downloads: &Unreached,
		directory: root.path.clone(),
		pinned: Vec::new(),
		equipped: Cell::new(false),
		reached: Cell::new(false),
	}
}
