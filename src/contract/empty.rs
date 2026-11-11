//! A section whose fields have not joined yet
//!
//! What a hand-off record holds for a section the simulation shows but no spec
//! has filled: an object that allows no field. The spec that fills a section
//! gives it a type of its own in place of this one.
//!
//! Definition only: the `Empty` type. It carries no field.
//!
//! ## Rules
//!
//! - It reads and writes as `{}`. Any field is refused.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Empty {}
