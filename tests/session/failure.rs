//! A failure shape, from its label and its value

use nsx::session::shape::Shape;

pub fn failure(label: &str, value: &str) -> Shape {
	Shape::Failure {
		label: label.to_string(),
		value: value.to_string(),
	}
}
