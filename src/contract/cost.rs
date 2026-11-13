//! What a judgment cost
//!
//! Definition only: the `Cost` type: whole cents, who priced them, and
//! whether the figure is illustrative. The one provenance sits beside it.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const PROVENANCE: &str = "console";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Cost {
	pub cents: u64,
	#[schemars(extend("const" = PROVENANCE))]
	pub provenance: String,
	pub illustrative: bool,
}
