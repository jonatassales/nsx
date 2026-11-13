//! What the team asked
//!
//! Definition only: the `Asked` type and its two values: `play` and `pause`.
//!
//! ## Rules
//!
//! - Any other value is refused: nsx never guesses what the team meant.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
#[schemars(inline)]
pub enum Asked {
	Play,
	Pause,
}
