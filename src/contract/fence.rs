//! The `runtime.md` fence
//!
//! The one `langdrift` JSON block a `runtime.md` holds: the sized plan the
//! team is asked to authorize, in a form a run can apply. The plan drafts it
//! and the run reads it. It carries only what the simulation shows.
//!
//! Definition only: the `Fence` type. It carries the `version`, the
//! `runtimeId` and the `revision`, the bound as `cpuMillis` and `memoryMiB`,
//! the suggested `model` and `effort`, and the `components`, each a
//! `Component`. The version, the bounds and the efforts sit beside it, for
//! its schema and its checks. Its functions are the files in `fence/`.
//!
//! ## Rules
//!
//! - Version 3 is the only version.
//! - A stack is `Rust`, `Node.js`, `NATS`, `Redis` or `Next.js`. A role is
//!   `api`, `broker`, `cache` or `web`. An effort is `low`, `medium` or `high`.
//! - `model` and `effort` are the judge's suggestion, kept as a
//!   recommendation. The choice a run applies lives in `intent.json`.
//! - The bound and the shares are whole millicores and MiB.
//! - The fence has no uid, project id, name, budget, env file, passed
//!   variable, Dockerfile, readiness or migration field, and no gpu or disk.
//! - The sizing report beside the fence in `runtime.md` is not part of the
//!   fence (spec 0055).
//!
//! Spec 0077.

pub mod parse;
pub mod validate;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::component::Component;

pub const VERSION: i64 = 3;

pub const SCHEMA: &str =
	"https://langdrift.md/schemas/configuration/runtime.schema.json";

pub const OPENING: &str = "```langdrift\n";

pub const CLOSING: &str = "\n```";

pub const MAXIMUM_CPU_MILLIS: i64 = 64_000;

pub const MAXIMUM_MEMORY_MIB: i64 = 1_048_576;

pub const MAXIMUM_COMPONENTS: usize = 16;

pub const EFFORTS: [&str; 3] = ["low", "medium", "high"];

// The figures are whole numbers of any size, so that `validate` can name a
// figure out of its bound instead of the decoding refusing it unnamed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Runtime fence",
	description = concat!(
		"The langdrift fence of runtime.md, version 3: the runtime's id and ",
		"revision, the bound in millicores and MiB, the suggested model and ",
		"effort, and per component its path, stack, role, port, image, share ",
		"and replicas. Shares summing within the bound, distinct ports and ",
		"images and unique paths are rules JSON Schema cannot express; ",
		"nsx enforces them.",
	),
	extend("$id" = SCHEMA),
)]
pub struct Fence {
	#[schemars(extend("const" = VERSION))]
	pub version: i64,
	#[schemars(pattern(r"^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$"))]
	pub runtime_id: String,
	#[schemars(range(min = 1))]
	pub revision: i64,
	#[schemars(range(min = 1, max = MAXIMUM_CPU_MILLIS))]
	pub cpu_millis: i64,
	#[serde(rename = "memoryMiB")]
	#[schemars(range(min = 1, max = MAXIMUM_MEMORY_MIB))]
	pub memory_mib: i64,
	#[schemars(length(min = 1))]
	pub model: String,
	#[schemars(extend("enum" = EFFORTS))]
	pub effort: String,
	#[schemars(length(min = 1, max = MAXIMUM_COMPONENTS))]
	pub components: Vec<Component>,
}
