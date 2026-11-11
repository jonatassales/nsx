//! A comment line
//!
//! One function: it takes a line's id and the words of a comment, and gives
//! back the line, in the comment kind and shape.
//!
//! ## Rules
//!
//! - The words come without the `# ` that opens a comment: the layout adds it.
//!
//! Spec 0077.

use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Line {
	pub fn comment(id: &str, text: &str) -> Self {
		Self {
			id: String::from(id),
			kind: Kind::Comment,
			shape: Shape::Comment {
				text: String::from(text),
			},
		}
	}
}
