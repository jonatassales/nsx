//! Safe Seat in the run
//!
//! Safe Seat keeps a person writing a part of every ticket by hand. In a run
//! that is the dev ticket: a stage between Implement and Review that always
//! belongs to the person. The run waits while the person writes the reserved
//! part, makes no call to the coding agent meanwhile, and states after the
//! checks pass that Safe Seat was met.
//!
//! - `Part`: the part a person writes, as the dev-ticket stage of the run
//! - `Verdict`: whether Safe Seat was met
//!
//! ## Rules
//!
//! - Every run that reaches the dev ticket plays it. No choice adds or
//!   removes it.
//! - No share, percentage, line count or reserved-part path is recorded.
//! - Nothing about Safe Seat is on the Plan card, and there are no Safe Seat
//!   rules or overrides to configure.
//! - Implement, its report and `code-review` are specs 0058 and 0059's. The
//!   checks are spec 0064's.
//!
//! Spec 0080.

pub mod error;
pub mod part;
pub mod verdict;
