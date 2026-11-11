//! The command that sets up `.langdrift/`
//!
//! `nsx init` creates LangDrift's own folder in the client's project, with
//! only what the plan and the run need. It asks nothing and prints nothing:
//! the simulation shows no line for it.
//!
//! Definition only: the `Init` type, the command, which takes no argument and
//! no flag. Its functions are the files in `init/`.
//!
//! ## Rules
//!
//! - The questions and flags of the old setup are removed: the envelope and
//!   the budget come from the design doc, where the team set them.
//! - It works in the working directory.
//!
//! Spec 0077.

pub mod execute;

use clap::Args;

#[derive(Debug, Args)]
pub struct Init;
