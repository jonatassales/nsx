//! The machine's clock
//!
//! The clock the binary reads: this machine's own time.
//!
//! Definition only: the `Watch` type. Its function is the file in `watch/`.
//!
//! Spec 0080.

pub mod now;

#[derive(Debug, Clone, Copy, Default)]
pub struct Watch;
