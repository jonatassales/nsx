//! The workflow and its human checkpoints
//!
//! The seven stages a run plays, in order, the choice the team makes for one
//! run, the waits where the owner confirms a stage, and how the run ends.
//! `nsx agent plan <ticket>` records the workflow and its default choice.
//! `nsx agent run` applies the choice the team authorized.
//!
//! - `Workflow`: the seven stages in order, and the default choice
//! - `Entry`: one stage as the workflow file lists it
//! - `Stage`: one stage of this run and the state it has reached
//! - `Choice`: the model, the effort and the stages a person confirms
//! - `Checkpoint`: a ticked stage, waiting for the owner or confirmed
//! - `Ending`: for review or to merge, as the Review tick decides
//!
//! ## Simulation
//!
//! - Card: Workflow · HUMAN IN THE LOOP
//! - Card: Workflow · MODEL
//! - Card: Workflow · EFFORT
//!
//! ## Rules
//!
//! - Every `nsx agent run` plays the stages from Planning, in order, after
//!   its gate. Nothing resumes: a later run is a new run from the gate.
//! - A checkpoint is answered inside `nsx agent run`: no other command, no
//!   second run and no prompt line.
//! - This folder runs no skill and no effort hook, prices nothing and prints
//!   no `ready` line: specs 0058, 0059, 0060, 0056 and 0063 do.
//!
//! Spec 0080.

pub mod checkpoint;
pub mod choice;
pub mod ending;
pub mod entry;
pub mod error;
pub mod keyboard;
pub mod reader;
pub mod stage;
pub mod workflow;
