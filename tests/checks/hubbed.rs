//! The scripted forge, with its pull request at the simulation's commit

use crate::simulation::fake::{forged::COMMIT, hub::Hub};

pub fn hubbed() -> Hub {
	Hub {
		commit: String::from(COMMIT),
		..Hub::default()
	}
}
