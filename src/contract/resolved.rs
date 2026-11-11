//! One row of the Plan card's COMPONENTS
//!
//! A component as the plan resolved it, as `plan.json` carries it.
//!
//! Definition only: the `Resolved` type. It carries the `path`, and once the
//! component is resolved its `manifest`, `stack`, `role`, `port`, `image`,
//! `cpuMillis` and `memoryMiB`. The five manifests sit beside the type.
//!
//! ## Rules
//!
//! - `path` is required: it is the placeholder the card shows from the
//!   `resolve` line on. Every other field is absent until its line prints.
//! - `stack` and `role` are spelled as the fence spells them.
//! - No other field is allowed.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::component::{ROLES, STACKS};

pub const MANIFESTS: [&str; 5] = [
	"next.config.ts",
	"go.mod",
	"package.json",
	"nats.conf",
	"redis.conf",
];

#[derive(
	Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Resolved {
	#[schemars(length(min = 1))]
	pub path: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", extend("enum" = MANIFESTS))]
	pub manifest: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", extend("enum" = STACKS))]
	pub stack: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", extend("enum" = ROLES))]
	pub role: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "i64", range(min = 1, max = 65535))]
	pub port: Option<i64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", length(min = 1))]
	pub image: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "i64", range(min = 1))]
	pub cpu_millis: Option<i64>,
	#[serde(
		default,
		rename = "memoryMiB",
		skip_serializing_if = "Option::is_none"
	)]
	#[schemars(with = "i64", range(min = 1))]
	pub memory_mib: Option<i64>,
}
