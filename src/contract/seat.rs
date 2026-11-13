//! The Safe Seat section of `run.json`
//!
//! The `safeSeat` section of `run.json`: the run's statement that it kept
//! the person's seat, and when it decided so.
//!
//! Definition only: the `Seat` type. It carries `safeSeat` and `decidedAt`.
//!
//! ## Rules
//!
//! - `safeSeat` is `met`: a run that did not meet it writes no section.
//! - It never carries a share, a percentage or a line count.
//! - No other field is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MET: &str = "met";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Seat {
	#[schemars(extend("const" = MET))]
	pub safe_seat: String,
	#[schemars(length(min = 1))]
	pub decided_at: String,
}
