//! The agent skills and their reports
//!
//! The skills the workflow's stages follow, and the one line each stage
//! prints. A skill ends by writing a small JSON report of figures. nsx checks
//! the report and prints the stage's line from its figures, never from
//! whatever the agent printed last.
//!
//! - `Skill`: a skill LangDrift ships, laid down by `nsx init`
//! - `Report`: a stage's checked report, and the one line it gives
//!
//! ## Rules
//!
//! - One paid call stands behind each stage line. `implement · tdd` is one
//!   stage with one line.
//! - The shapes of the five stage reports live in `contract/` (spec 0049).
//! - Starting the coding agent is spec 0058's. The order of the stages and
//!   their checkpoints is spec 0057's.
//!
//! Spec 0080.

pub mod error;
pub mod report;
pub mod skill;
