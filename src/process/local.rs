//! The runner on this machine
//!
//! The `Runner` the commands use outside tests: it starts a real program on
//! the person's machine.
//!
//! Definition only: the `Local` type. Its function is the file in `local/`.
//!
//! Shared: used by `driver`, `local` and `delivery`.

pub mod run;

#[derive(Debug, Clone, Copy, Default)]
pub struct Local;
