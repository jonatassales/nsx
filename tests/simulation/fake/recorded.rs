//! The recorded judge
//!
//! A judge that answers each question with a text recorded beforehand. It is
//! the judge the tests play with, the golden test first: no test reaches a
//! real model.
//!
//! Definition only: the `Recorded` type: the answer to give to each question,
//! as the judge would write it. Its implementation of `Judge` is the file in
//! `recorded/`.
//!
//! ## Rules
//!
//! - It gives its texts back as they are. The asking function reads and
//!   checks them like any judge's, so a recorded answer can be a wrong one.
//!
//! Spec 0078.

mod ask;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Recorded {
	pub criteria: String,
	pub episodes: String,
	pub suggestion: String,
}
