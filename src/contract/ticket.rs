//! The Ticket card
//!
//! The `ticket` section of `plan.json`: what the Ticket card shows of the
//! ticket a plan was asked for.
//!
//! Definition only: the `Ticket` type. It carries the card's header: the
//! ticket's bare `id`, its `number`, its `slug`, its `status` and its `title`;
//! and its `links`, the four linked artifacts.
//!
//! ## Rules
//!
//! - The header's five fields are always present. `id` and `slug` are never
//!   empty.
//! - `number`, `status` and `title` are `null` when the ticket does not state
//!   them, never an empty text.
//! - The header is written before the plan's first line.
//! - `links` is absent until the plan's `links` line prints, then holds the
//!   four documents in the order PRD, design doc, spec, guardrail.
//! - No other field is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{linked::Linked, nullable::nullable};

// A field the ticket does not state is required all the same, and `null`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(
	inline,
	description = concat!(
		"The Ticket card. Its header is written before the plan's first ",
		"line: the ticket's bare id, number, slug, status and title. A ",
		"number, status or title the ticket does not state is null. Its ",
		"links are written when the plan's links line prints.",
	),
)]
pub struct Ticket {
	#[schemars(length(min = 1))]
	pub id: String,
	#[serde(deserialize_with = "Option::deserialize")]
	#[schemars(required, transform = nullable, length(min = 1))]
	pub number: Option<String>,
	#[schemars(length(min = 1))]
	pub slug: String,
	#[serde(deserialize_with = "Option::deserialize")]
	#[schemars(required, transform = nullable, length(min = 1))]
	pub status: Option<String>,
	#[serde(deserialize_with = "Option::deserialize")]
	#[schemars(required, transform = nullable, length(min = 1))]
	pub title: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Vec<Linked>")]
	pub links: Option<Vec<Linked>>,
}
