//! The team's offer
//!
//! What the team offers on the console's Settings → AI Models: per provider,
//! the models and the efforts its account may use. The console answers it
//! through the tool `langdrift.account.offer`.
//!
//! Definition only: the `Offer` type: its `providers`, each an `Offered`.
//! The `$id` it is published under sits beside it.
//!
//! ## Rules
//!
//! - The console sends each provider once. JSON Schema cannot say so, and nsx
//!   reads every entry that names a model.
//! - The efforts are the words the team typed, not a closed set: nsx keeps
//!   only the efforts it knows.
//! - No other field is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::offered::Offered;

pub const SCHEMA: &str = "https://langdrift.md/schemas/offer/offer.schema.json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Team offer",
	description = concat!(
		"What the team offers on the console's Settings → AI Models: per ",
		"provider, the models and the efforts its account may use. The ",
		"console sends each provider once.",
	),
	extend("$id" = SCHEMA),
)]
pub struct Offer {
	pub providers: Vec<Offered>,
}
