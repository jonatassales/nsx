//! The judge of the Analysis card
//!
//! The `judge` section of `plan.json`'s `analysis`: why the judge could not
//! be asked, when it could not. The card shows it as its JUDGE field and its
//! header.
//!
//! Definition only: the `Availability` type. It carries `unavailable`, the
//! console's reason. The reasons it may give sit beside it.
//!
//! ## Rules
//!
//! - The section is absent while the judge can be asked. It is present only
//!   when the plan stopped before the judge's first question.
//! - `unavailable` has one value, `model-key-missing`: the team has no
//!   Anthropic key on the console's Settings → AI Models.
//! - No other field is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// The console's reason when the team has no Anthropic key.
pub const MODEL_KEY_MISSING: &str = "model-key-missing";

pub const UNAVAILABLE: [&str; 1] = [MODEL_KEY_MISSING];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Availability {
	#[schemars(extend("enum" = UNAVAILABLE))]
	pub unavailable: String,
}
