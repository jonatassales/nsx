//! Two checks: one passed, and one where a test says it stands

use nsx::checks::{
	standing::Standing::{self, Passed},
	suite::Suite,
};

use crate::checks::listed::listed;

pub fn checked(last: Standing) -> Suite {
	Suite {
		checks: listed(&[("build", Passed), ("test", last)]),
	}
}
