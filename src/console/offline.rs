//! The console of a project with no connection
//!
//! A project that recorded no secret plans and runs offline: nothing is
//! sent, and every call answers that there is no console.
//!
//! Definition only: the `Offline` type. Its function is the file in
//! `offline/`.
//!
//! Spec 0081.

pub mod report;

#[derive(Debug, Clone, Copy, Default)]
pub struct Offline;
