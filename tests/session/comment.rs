//! A comment shape, from its words

use nsx::session::shape::Shape;

pub fn comment(text: &str) -> Shape {
	Shape::Comment {
		text: text.to_string(),
	}
}
