//! The effort hook
//!
//! A built-in step of `nsx agent run`, run once at the start of Implement. It
//! records the effort the run implements at, and implement may start only for
//! a run whose effort record matches the choice the team applied. It is named
//! for what it guards and is not a hook system: no other step of the run can
//! be hooked.
//!
//! - `Effort`: the effort, recorded before implement may start
//!
//! ## Rules
//!
//! - It does not choose the model or the effort: `flow/` applies the choice
//!   and `judgment/` suggests it.
//! - It does not run implement or print its line: `driver/` and `skills/` do.
//! - No script hook runs in a stage, and no line reports a script's exit.
//!
//! ## Open
//!
//! - Whether the hook ships the `llm-effort-analyzer` skill and its guard into
//!   `.langdrift/`, instead of this built-in step, is the owner's decision.

pub mod effort;
pub mod error;
