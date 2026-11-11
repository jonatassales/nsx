//! Whether Safe Seat was met
//!
//! The run's statement that it kept the person's seat. It says whether the
//! floor was met, never a share.
//!
//! Definition only: the `Verdict` type, which carries whether Safe Seat was
//! met and when that was decided. Its functions are the files in `verdict/`.
//!
//! ## Simulation
//!
//! - Card: Workflow · SAFE SEAT: a dash until `✓ Safe Seat  met` has printed,
//!   then `met`, in the success tone
//!
//! ## Rules
//!
//! - A run that stops before the checks pass has no verdict: no line prints
//!   and SAFE SEAT stays unset.
//! - The verdict goes into the run's record (spec 0065).
//! - It never carries a share, a percentage or a line count.
//!
//! ## Open
//!
//! - What a run that does not meet Safe Seat prints. The simulation never
//!   shows one.
//!
//! Spec 0080.

pub mod card;
pub mod line;
pub mod reach;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
	pub met: bool,
	pub decided_at: String,
}
