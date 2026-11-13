//! The team's Play
//!
//! The team's authorization of the recorded intent, pressed in the console.
//! In a connected project, an `nsx agent run` whose intent is still proposed
//! waits for it silently, before the run's header. A Play carries the choice
//! it authorizes, so that the team authorizes what was proposed and nothing
//! else.
//!
//! Definition only: the `Play` type, which carries the intent as `intent.json`
//! held it when the Play matched, so that the run authorizes that intent and
//! no earlier reading of it. The pace of a wait sits beside it. Its function
//! is the file in `play/`.
//!
//! ## Rules
//!
//! - nsx records the team's authorization, it never decides it. After a
//!   matching Play, `status: authorized` is written into `intent.json`, and
//!   nothing else into it (`handoff/authorization/grant.rs`).
//! - A Play applies only when its choice equals the revision, model, effort
//!   and checkpoints of `intent.json`. One that differs is ignored.
//! - A Play never changes a budget or a reservation. The gate still decides
//!   whether the paid run goes ahead.
//! - An intent authorized by hand needs no Play: the run starts at once.
//! - Waiting prints no line (the owner's exception to ADR 001). The run's
//!   header prints only once the team has authorized.

pub mod wait;

use std::time::Duration;

use crate::contract::intent::Intent;

pub const BEAT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Play {
	pub intent: Intent,
}
