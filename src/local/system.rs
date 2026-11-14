//! The system nsx runs on
//!
//! What differs between the machines nsx is built for when it sets up the
//! local runtime: how Docker is started, and what a missing one needs.
//!
//! Definition only: the `System` type, with two systems: `Mac` and `Linux`.
//! Its function is the file in `system/`.
//!
//! ## Rules
//!
//! - On a Mac, nsx can start Docker itself, and install a container runtime
//!   with no password.
//! - On Linux, starting or installing Docker needs root: nsx says the
//!   command and never asks for a password.

pub mod here;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum System {
	Mac,
	Linux,
}
