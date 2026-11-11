//! One component of the fence
//!
//! A piece of the team's software the runtime runs, as the Plan card's
//! COMPONENTS rows show it: where it lives, what it is, where it listens,
//! which image runs it and what share of the bound it gets.
//!
//! Definition only: the `Component` type. It carries the `path`, the `stack`,
//! the `role`, the `port`, the `image`, the proposed share as `cpuMillis` and
//! `memoryMiB`, and the `replicas`. The closed sets of stacks and roles, and
//! the range of a port, sit beside it, for the fence's schema and its checks.
//!
//! ## Rules
//!
//! - A stack is `Go`, `Node.js`, `NATS`, `Redis` or `Next.js`. A role is
//!   `api`, `broker`, `cache` or `web`.
//! - A port is a whole number from 1 to 65535.
//! - The share is whole millicores and MiB, and `replicas` is 1.
//! - Which of its values break a rule is the fence's `validate` to say.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const STACKS: [&str; 5] = ["Go", "Node.js", "NATS", "Redis", "Next.js"];

pub const ROLES: [&str; 4] = ["api", "broker", "cache", "web"];

pub const PORTS: std::ops::RangeInclusive<i64> = 1..=65535;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Component {
	#[schemars(length(min = 1))]
	pub path: String,
	#[schemars(extend("enum" = STACKS))]
	pub stack: String,
	#[schemars(extend("enum" = ROLES))]
	pub role: String,
	#[schemars(range(min = 1, max = 65535))]
	pub port: i64,
	#[schemars(length(min = 1))]
	pub image: String,
	#[schemars(range(min = 1))]
	pub cpu_millis: i64,
	#[serde(rename = "memoryMiB")]
	#[schemars(range(min = 1))]
	pub memory_mib: i64,
	#[schemars(extend("const" = 1))]
	pub replicas: i64,
}
