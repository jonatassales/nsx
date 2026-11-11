//! What a run leaves behind
//!
//! A run leaves two records of its own in the runtime's folder, and nothing
//! else: the usage record, with what each paid stage call used, and the
//! run's record, with the Workflow card after the run and the evidence of
//! each line.
//!
//! - `Clock`: the time, as a trait, and `Watch`, the machine's own clock
//! - `Id`: the run's id
//! - `Record`: the one writer of `run.json`
//! - `Usage`: the usage record, `usage.json`, and the completed line
//!
//! ## Rules
//!
//! - Evidence and reports carry no secret.
//! - Every value is in units. What is not known is `null`, never zero.
//! - The money is not copied here: the money ledger is `money`'s (spec
//!   0056). The episode is `rag`'s (spec 0052).
//! - This folder decides the content of no section but its own.
//!
//! Spec 0080.

pub mod clock;
pub mod error;
pub mod id;
pub mod record;
pub mod usage;
pub mod used;
pub mod watch;
