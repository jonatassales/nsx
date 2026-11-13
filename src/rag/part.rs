//! One component an episode's run ran
//!
//! What the judge reads of a past run's components to say whether it shares
//! the stack and scale.
//!
//! Definition only: the `Part` type: the component's path, its stack and its
//! role.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Part {
	pub path: String,
	pub stack: String,
	pub role: String,
}
