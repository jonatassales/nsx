//! The problem of one field breaking one rule
//!
//! One function: it takes a field, as a dotted path, and the rule it breaks,
//! in a few words, and gives back the problem.

use crate::contract::problem::Problem;

impl Problem {
	pub fn at(field: &str, rule: &str) -> Self {
		Self {
			field: String::from(field),
			rule: String::from(rule),
		}
	}
}
