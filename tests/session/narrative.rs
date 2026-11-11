//! A narrative shape, from its words

use nsx::session::shape::Shape;

pub fn narrative(text: &str) -> Shape {
	Shape::Narrative {
		text: text.to_string(),
	}
}
