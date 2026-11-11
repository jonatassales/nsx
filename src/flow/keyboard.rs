//! The run's standard input
//!
//! The reader the binary answers from: the terminal the run was typed in, or
//! whatever was piped into it.
//!
//! Definition only: the `Keyboard` type. Its function is the file in
//! `keyboard/`.
//!
//! Spec 0080.

pub mod line;

#[derive(Debug, Clone, Copy, Default)]
pub struct Keyboard;
