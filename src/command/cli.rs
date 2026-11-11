//! The command line of nsx
//!
//! What a person may type after `nsx`: `init`, or `agent` with `plan` or
//! `run`. The tool a person installs is the tool the simulation shows.
//!
//! Definition only: the `Cli` type, the parsed command line, which is either
//! `init` or the `agent` family. Its functions are the files in `cli/`.
//!
//! ## Rules
//!
//! - nsx has three commands: `nsx init`, `nsx agent plan <ticket>` and
//!   `nsx agent run`. Every other command is removed.
//! - The global flags are removed: the project is found from the working
//!   directory.
//! - `--version` is removed, because the session does not show it. `--help`
//!   stays as clap prints it until the owner decides (spec 0077), and the
//!   `help` command is removed.
//!
//! ## Open
//!
//! - Whether `--help` stays and `--version` returns as conventions outside
//!   the session. The owner decides.
//! - What nsx does with a command it does not have. The spec names no such
//!   case; the stop rule covers a refusal the simulation does not show.
//!
//! Spec 0077.

pub mod execute;

use clap::Parser;

use crate::command::{agent::Agent, init::Init};

#[derive(Debug, Parser)]
#[command(name = "nsx", disable_help_subcommand = true)]
pub enum Cli {
	/// Set up `.langdrift/` in this project
	Init(Init),
	/// Plan a ticket, then run the plan the team authorized
	#[command(subcommand)]
	Agent(Agent),
}
