//! A status shape, from its label and its value

use nsx::session::shape::Shape;

pub fn status(label: &str, value: &str) -> Shape {
	Shape::Status {
		label: label.to_string(),
		value: value.to_string(),
	}
}
