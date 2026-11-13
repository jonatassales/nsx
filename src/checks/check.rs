//! One check
//!
//! One check of the client's CI on the run's pull request, as the forge
//! lists it.
//!
//! Definition only: the `Check` type: the check's name and where it stands.

use crate::checks::standing::Standing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Check {
	pub name: String,
	pub standing: Standing,
}
