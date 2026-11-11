//! The client's CI checks
//!
//! Once the pull request is open, the run waits for the client's own CI. It
//! reads the pull request's checks from the forge, prints their names once it
//! sees them, waits until every one has completed, and lets the run go on to
//! Safe Seat only when all passed. The checks are the client's: nsx does not
//! define, run or change them.
//!
//! - `Suite`: the checks of the run's pull request
//! - `Check`: one check, with its name and where it stands
//! - `Standing`: running, passed or failed
//! - `Error`: what can go wrong here
//!
//! ## Rules
//!
//! - The run's order: `pullRequest`, then `checks`, then `checksPassed`, then
//!   `safeSeatMet`.
//! - A run paused at the gate reads no checks.
//! - The forge is the one delivery already uses (spec 0063), so a person
//!   installs nothing more.
//! - `guardrails` is a check of the client's CI, not something nsx runs.
//!
//! Spec 0081.

pub mod check;
pub mod error;
pub mod standing;
pub mod suite;
