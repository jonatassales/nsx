//! The person who interrupts as soon as the command first waits
//!
//! Each wait is over the second time it is looked at: once the command has
//! had its turn and is waiting on something outside itself.

mod wait;

#[derive(Debug, Clone, Copy, Default)]
pub struct Hasty;
