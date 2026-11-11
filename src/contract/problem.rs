//! One problem a validation found
//!
//! What a check says when a shape breaks a rule. A validation gives back a
//! list of them, so that the plan can count them.
//!
//! Definition only: the `Problem` type. It carries the field, as a dotted path
//! such as `components[1].port`, and the rule in a few words. Its function is
//! the file in `problem/`.
//!
//! Spec 0077.

pub mod at;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
	pub field: String,
	pub rule: String,
}
