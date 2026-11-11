//! The two commands that play the session
//!
//! `nsx agent plan <ticket>` drafts a plan and proposes it to the team.
//! `nsx agent run` applies what the team authorized. Together they are the
//! session the simulation shows, phase 1 and phase 2.
//!
//! Definition only: the `Agent` type, which is either the plan, with the
//! ticket's bare id, or the run, with no argument. Its functions are the
//! files in `agent/`.
//!
//! ## Rules
//!
//! - The family has these two commands and no other. A human checkpoint is
//!   answered inside `nsx agent run`, never by a command of its own.
//!
//! Spec 0077.

pub mod execute;

use clap::Subcommand;

use crate::command::{plan::Plan, run::Run};

#[derive(Debug, Subcommand)]
pub enum Agent {
	/// Draft a plan for a ticket and propose it to the team
	Plan(Plan),
	/// Apply the plan the team authorized
	Run(Run),
}
