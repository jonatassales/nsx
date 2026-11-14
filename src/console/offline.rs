//! The console of a project with no connection
//!
//! A project that recorded no secret plans and runs offline: nothing is
//! sent, and every call answers that there is no console.
//!
//! Definition only: the `Offline` type. Its implementation of `Client` is the
//! file in `offline/`.

pub mod client;

#[derive(Debug, Clone, Copy, Default)]
pub struct Offline;
