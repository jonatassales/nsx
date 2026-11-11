//! The run's delivery
//!
//! The run delivers its own work with the repository's tools, `git` and `gh`.
//! It starts the ticket's branch before the first stage, pushes it once the
//! implementation is complete, opens a draft pull request and, at the very
//! end, marks it ready.
//!
//! - `Forge`: the forge, as a trait
//! - `Github`: the forge reached through `git` and `gh`
//! - `Branch`: the run's branch, started and pushed
//! - `Pull`: the pull request, opened as a draft and marked ready
//! - `Readiness`: where a pull request stands
//! - `Error`: what can go wrong here
//!
//! ## Rules
//!
//! - nsx never merges: the owner decides.
//! - nsx commits nothing itself. The stages and the person write the commits.
//! - Nothing here happens in an over-budget run.
//! - Every failure follows the stop rule, with no line of its own, and the
//!   runtime, when up, is taken down silently (spec 0062).
//!
//! ## Open
//!
//! - Who delivers. Until the owner decides, nsx does. The alternative is that
//!   the coding agent delivers and nsx only reports.
//!
//! Spec 0081.

pub mod branch;
pub mod error;
pub mod forge;
pub mod github;
pub mod pull;
pub mod readiness;
