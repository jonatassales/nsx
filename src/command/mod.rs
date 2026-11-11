//! The three commands of nsx
//!
//! A person types `nsx init`, `nsx agent plan <ticket>` and `nsx agent run`,
//! and nothing else. Each command prints exactly the simulation's lines for
//! its phase, in the simulation's order, and nothing more. The plan and the
//! run are written as scripts: each step calls the folder that owns the line.
//!
//! - `Cli`: the command line, which is `init` or `agent`
//! - `Init`: `nsx init`, which creates `.langdrift/` and prints nothing
//! - `Agent`: the `agent` family, which is the plan or the run
//! - `Plan`: `nsx agent plan <ticket>`, as its steps in order
//! - `Run`: `nsx agent run`, as its steps in order
//! - `Session`: what a command needs from outside itself
//! - `Brief`: what the plan has read before its first line
//!
//! ## Rules
//!
//! - Every command and flag the simulation does not show is removed
//!   (ADR 0005).
//! - The typed command is never echoed: the person's shell shows it once.
//! - The plan and the run find the project from any folder inside it. The
//!   root is the nearest folder, from the working directory up, that holds
//!   `.langdrift/`. None is a stop.
//!
//! Spec 0077.

pub mod agent;
pub mod applied;
pub mod brief;
pub mod cli;
pub mod desk;
pub mod init;
pub mod plan;
pub mod resolved;
pub mod run;
pub mod session;
pub mod work;
