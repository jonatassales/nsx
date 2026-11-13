//! No judge
//!
//! The judge of a project that has none connected. The real judge is reached
//! through the console; until it is, the binary asks this one, and a plan
//! stops at its first question.
//!
//! Definition only: the `Absent` type. Its implementation of `Judge` is the
//! file in `absent/`.
//!
//! ## Rules
//!
//! - It invents no answer: a plan never rests on a judgment nobody made.

pub mod ask;

#[derive(Debug, Clone, Copy, Default)]
pub struct Absent;
