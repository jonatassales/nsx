//! The envelope
//!
//! The bound on cpu and memory that the team set in the design doc. The plan
//! states it as its bound, with the design doc as its provenance.
//!
//! Definition only: the `Envelope` type: cpu in millicores, memory in MiB,
//! and its source, the design doc's path. Who set it is always the team, so
//! it is not kept.
//! Its functions are the files in `envelope/`.
//!
//! ## Rules
//!
//! - The design doc is the one source of the bound. No CLI flag, recorded
//!   input, mirror, default or per-execution ceiling takes part.
//! - "The team" is the design doc's author as a group. No identity is read or
//!   recorded for it.
//! - Whole units, as plain integers: millicores and MiB.
//! - The bound goes into the `runtime.md` draft with its source, and into the
//!   plan's card data: BOUNDED is set here, from the `bounds` line. HEADROOM
//!   belongs to `runtime/`.
//! - A design doc without the envelope stops the plan (the stop rule). The
//!   plan never sizes against an invented bound.

pub mod card;
pub mod line;
pub mod of;

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
	pub cpu_millis: i64,
	pub memory_mib: i64,
	pub source: PathBuf,
}
