//! The ticket's four links
//!
//! The documents a ticket links: one PRD, one design doc, one spec and one
//! guardrail. The plan reads these documents and follows no other link. Each
//! is named by its short id, and they always come in the order PRD, design
//! doc, spec, guardrail, whatever order the ticket wrote them in.
//!
//! Definition only: the `Links` type: four links, one per kind, each a `Link`
//! with its file name, its short id and its project-relative path. Its
//! functions are the files in `links/`.
//!
//! ## Rules
//!
//! - A ticket links exactly one document of each kind.
//! - The PRD, the design doc and the spec are the client's and sit in the
//!   client's artifact folder. The guardrail is LangDrift's and sits in
//!   `.langdrift/harness/guardrails/`.
//! - The guardrail link is handed to `harness/`, which reads the document.

pub mod card;
pub mod line;
pub mod parse;

use crate::request::link::Link;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Links {
	pub prd: Link,
	pub design: Link,
	pub spec: Link,
	pub guardrail: Link,
}
