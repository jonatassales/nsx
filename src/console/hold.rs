//! A hold before a stage
//!
//! The team's Pause, as a run applies it. The run holds before the next
//! stage's work starts, until the team presses Play again. The stage in
//! progress finishes: nothing is interrupted, and nothing more is paid while
//! the run holds.
//!
//! Definition only: the `Hold` type, the hold point a run passes before each
//! stage's work. Its function is the file in `hold/`.
//!
//! ## Rules
//!
//! - The hold points are the stages nsx runs, in order: Planning, Spec,
//!   Tickets, Implement, Dev ticket and Review. Each is right before the
//!   stage's work starts: its first paid call or, for the dev ticket, the
//!   person's part.
//! - For a ticked stage, the hold point follows `✓ confirmed`, so a person
//!   still answers the checkpoint first.
//! - A held run reports `held`. That is not the gate's `paused`: there the
//!   process ended and the owner decides.
//! - A held run continues the same run on Play. It neither replays a stage
//!   nor starts a new run.
//! - A Pause during Review's work has no stage left to hold before. The run
//!   ends as it would.
//! - Holding prints no line (the owner's exception to ADR 0005).

pub mod wait;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Hold;
