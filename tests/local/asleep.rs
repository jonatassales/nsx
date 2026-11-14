//! kind as a test wakes it: on a system, over a runner, with a Docker that
//! has not answered yet and no pause between two asks

use std::cell::Cell;
use std::time::Duration;

use nsx::files::root::Root;
use nsx::local::{kind::Kind, system::System};
use nsx::process::runner::Runner;

use crate::simulation::fake::unreached::Unreached;

pub fn asleep<'run>(
	programs: &'run dyn Runner,
	root: &Root,
	system: System,
) -> Kind<'run> {
	Kind {
		programs,
		downloads: &Unreached,
		directory: root.path.clone(),
		pinned: Vec::new(),
		equipped: Cell::new(false),
		system,
		pause: Duration::ZERO,
		awake: Cell::new(None),
		reached: Cell::new(false),
	}
}
