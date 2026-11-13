//! What asking the judge can refuse
//!
//! The errors of this folder, each one plain sentence, so that the stop rule
//! can print it as it is.
//!
//! Definition only: the `Error` type. Its function is the file in `error/`.

pub mod refused;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"nsx has no judge to ask yet: the plan stops here until the console's judge is connected."
	)]
	Unconnected,
	#[error("The console's judge did not answer: {cause}.")]
	Unavailable { cause: String },
	#[error("nsx could not read the judge's {question} answer: {cause}.")]
	Unread {
		question: &'static str,
		cause: crate::contract::error::Error,
	},
	#[error("nsx refuses the judge's {question} answer: {field} {rule}.")]
	Refused {
		question: &'static str,
		field: String,
		rule: String,
	},
}
