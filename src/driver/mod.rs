//! The stage driver
//!
//! Starts the team's coding agent once for each stage that has a skill:
//! Planning, Spec, Tickets, Implement and Review. The model and the effort
//! the run applies reach every call. The driver prints nothing itself: each
//! stage's one line belongs to the skills.
//!
//! - `Agent`: the coding agent, as a trait
//! - `Config`: `driver.json`, which says how the coding agent is started
//! - `Call`: one call for one stage, and its result
//! - `Tokens`: the input and output tokens a call used
//!
//! ## Rules
//!
//! - A run makes five calls, in the order Planning, Spec, Tickets, Implement,
//!   Review. It makes none while the person writes the dev ticket, and none
//!   in an over-budget run.
//! - No terminal line and no card field belongs to this folder.
//! - Nothing is retried and nothing resumes.
//! - The driver does not choose the model, the effort or the stages, price
//!   the tokens or keep the usage record.

pub mod agent;
pub mod call;
pub mod config;
pub mod ended;
pub mod error;
pub mod launcher;
pub mod paths;
pub mod tokens;
