//! A human checkpoint
//!
//! A ticked stage where the run stops before the stage's work and waits for
//! the owner. The owner answers inside the same `nsx agent run`: no other
//! command, no second run and no prompt line. Planning, Spec, Tickets and
//! Implement can have a checkpoint, and Planning always has one.
//!
//! Definition only: the `Checkpoint` type and its two states, waiting and
//! confirmed. Its functions are the files in `checkpoint/`.
//!
//! ## Rules
//!
//! - An unticked stage has no checkpoint: it runs without waiting.
//! - Review never has one. `code-review` runs without waiting even when
//!   Review is ticked: the tick only decides how the run ends.
//! - The Dev ticket's wait is not a checkpoint: it is `seat`'s.
//!
//! ## Open
//!
//! - What a declined checkpoint prints and records beyond the stop. The owner
//!   settled how it is declined on 2026-11-11: yes confirms, and anything
//!   else, or the end of the input, declines and stops the run.
//! - Who "the owner" is. nsx cannot tell who answers at the terminal, and
//!   records the role `owner`.

pub mod confirm;
pub mod line;
pub mod wait;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Checkpoint {
	Waiting { stage: String },
	Confirmed { stage: String },
}
