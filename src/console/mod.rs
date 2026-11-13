//! The console
//!
//! How a run reaches the team's console, and how the team's Play and Pause
//! reach a run. nsx sends each hand-off record to the console as it writes it
//! and reads the team's request in the console's answer. A project without
//! the connection plans and runs offline: the console is never required.
//!
//! - `Client`: the console, as a trait
//! - `Remote`: the console over the network, and `Offline`, the console of
//!   a project with no connection
//! - `Connection`: the console's origin and the secret, and `origin`, where
//!   nsx will send a secret
//! - `Error`: what reaching the console can refuse
//! - `Report`: the records written so far, sent with the run's status
//! - `Play`: the team's Play, which a run waits for before its header
//! - `Hold`: the hold before a stage's work, when the team paused the run
//!
//! ## Rules
//!
//! - The console is the owner's exception to ADR 0005. Nothing this folder
//!   does prints a line: the terminal prints exactly the simulation's lines,
//!   byte for byte, whether or not the console answers.
//! - The secret is never printed, logged or written into evidence.
//! - `held` is not the gate's `paused`. A held run waits for the team. A run
//!   the gate paused has ended.
//! - No command is added and nothing runs in the background.

pub mod client;
pub mod connection;
pub mod error;
pub mod hold;
pub mod offline;
pub mod origin;
pub mod play;
pub mod remote;
pub mod report;
