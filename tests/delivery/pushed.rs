//! The simulation's branch, pushed

use nsx::delivery::branch::Branch;

use crate::simulation::fake::forged::COMMIT;

pub fn pushed() -> Branch {
	Branch {
		name: String::from("feat/0031-book-a-demo-runtime"),
		commit: Some(String::from(COMMIT)),
	}
}
