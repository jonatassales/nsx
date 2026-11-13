//! The team's pending request
//!
//! What the team asked in the console and nsx has not applied yet: a play,
//! with the choice it authorizes, or a pause.
//!
//! Definition only: the `Requested` type. It carries `state` and, for a
//! play, `intent`.
//!
//! ## Rules
//!
//! - A play carries the authorized choice as `intent`. A pause carries
//!   nothing more.
//! - No other field is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{asked::Asked, choice::Choice};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Requested {
	pub state: Asked,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Choice")]
	pub intent: Option<Choice>,
}
