//! A result shape, from its label and its value

use nsx::session::shape::Shape;

pub fn result(label: &str, value: &str) -> Shape {
	Shape::Result {
		label: label.to_string(),
		value: value.to_string(),
	}
}
