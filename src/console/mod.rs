//! The console
//!
//! How a run reaches the team's console, and how the team's Play and Pause
//! reach a run. nsx sends each hand-off record to the console as it writes it;
//! the team's request in the console's answer joins with ticket 0433. A project
//! without the connection plans and runs offline: the console is never
//! required.
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
//! - Specs 0073 and 0076 are the owner's exceptions to ADR 0005. Nothing this
//!   folder does prints a line: the terminal prints exactly the simulation's
//!   lines, byte for byte, whether or not the console answers.
//! - The secret is never printed, logged or written into evidence.
//! - `held` (spec 0076) is not the gate's `paused` (spec 0056). A held run
//!   waits for the team. A run the gate paused has ended.
//! - No command is added and nothing runs in the background.
//!
//! Spec 0081.

pub mod client;
pub mod connection;
pub mod error;
pub mod hold;
pub mod offline;
pub mod origin;
pub mod play;
pub mod remote;
pub mod report;
