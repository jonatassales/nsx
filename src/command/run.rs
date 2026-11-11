//! The run command, phase 2 of the simulation
//!
//! `nsx agent run` applies the one recorded intent the team authorized. It
//! gates the money, plays the workflow's stages, brings the runtime up,
//! delivers a pull request and releases what the reservation did not spend.
//!
//! Definition only: the `Run` type, the command, which takes no argument and
//! no flag. Its functions are the files in `run/`.
//!
//! ## Rules
//!
//! - The run names nothing: the ticket, the model, the effort and the
//!   checkpoints are the recorded intent's.
//! - It applies the one intent whose status is authorized and that has no
//!   completed run. More than one is a stop. None is a stop, unless the
//!   project is connected to the console and one intent is proposed: then
//!   the run waits, printing nothing, for the team's Play (spec 0076).
//! - A completed intent is not runnable again. A stopped run's intent stays
//!   runnable and starts again from the gate.
//! - A person's answer is a line on the run's standard input, never a second
//!   command.
//!
//! ## Open
//!
//! - Which intent runs when several are authorized, and whether an intent
//!   may run a second time. The rules above stand until the owner decides.
//!
//! Spec 0077.

pub mod authorize;
pub mod call;
pub mod carry;
pub mod deliver;
pub mod execute;
pub mod follow;
pub mod halt;
pub mod host;
pub mod person;
pub mod play;
pub mod prepare;
pub mod remember;
pub mod reserve;
pub mod settle;
pub mod ship;
pub mod stages;

use clap::Args;

#[derive(Debug, Args)]
pub struct Run;
