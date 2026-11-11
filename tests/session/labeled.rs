//! A labeled shape, from its label and its value

use nsx::session::shape::Shape;

pub fn labeled(label: &str, value: &str) -> Shape {
	Shape::Labeled {
		label: label.to_string(),
		value: value.to_string(),
	}
}
