//! kind as a test reaches it: in a project, over the runner that starts
//! nothing

use std::cell::Cell;

use nsx::files::root::Root;
use nsx::local::kind::Kind;

use crate::simulation::fake::programs::Programs;

pub fn reached<'run>(programs: &'run Programs, root: &Root) -> Kind<'run> {
	Kind {
		programs,
		directory: root.path.clone(),
		reached: Cell::new(false),
	}
}
