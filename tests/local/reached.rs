//! kind as a test reaches it: in a project, over the runner that starts
//! nothing, with no tool to bring, no web to download from, and a Docker
//! that already answered

use std::cell::Cell;
use std::time::Duration;

use nsx::files::root::Root;
use nsx::local::{kind::Kind, system::System};

use crate::simulation::fake::{programs::Programs, unreached::Unreached};

pub fn reached<'run>(programs: &'run Programs, root: &Root) -> Kind<'run> {
	Kind {
		programs,
		downloads: &Unreached,
		directory: root.path.clone(),
		pinned: Vec::new(),
		equipped: Cell::new(false),
		system: System::Mac,
		pause: Duration::ZERO,
		awake: Cell::new(Some(true)),
		reached: Cell::new(false),
	}
}
