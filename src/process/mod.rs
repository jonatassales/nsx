//! Running other programs
//!
//! nsx starts programs it does not own: the team's coding agent, `git` and
//! `gh`, and `kind`, `kubectl` and Docker. This folder is the one place that
//! starts them, so that each is started the same way and a test can script any
//! of them.
//!
//! - `Runner`: running a program, as a trait
//! - `Local`: the runner that starts the program on this machine
//! - `Environment`: the few variables a program is given
//! - `Program`: a program to run: its name, arguments, environment and folder
//! - `Output`: what a program printed and how it ended
//! - `Error`: what can go wrong with a program
//!
//! ## Rules
//!
//! - A program nsx starts never inherits nsx's environment. It is given a
//!   small, explicit set of variables.
//! - The coding agent is started directly, never through a shell, and its
//!   arguments are never split again.
//! - What a program prints is not printed by nsx: only the simulation's lines
//!   reach the terminal.
//!
//! Shared: used by `driver`, `local` and `delivery`.

pub mod environment;
pub mod error;
pub mod local;
pub mod output;
pub mod program;
pub mod runner;
