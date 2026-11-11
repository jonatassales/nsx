//! The simulation's pull request, where a test says it stands

use nsx::delivery::{pull::Pull, readiness::Readiness};

pub const TITLE: &str = "feat(demo): run book a demo as a microruntime";

pub fn pulled(state: Readiness) -> Pull {
	Pull {
		number: 19,
		title: String::from(TITLE),
		state,
	}
}
