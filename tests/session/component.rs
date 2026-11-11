//! A component's row, from its path, its stack and role, and its share

use nsx::session::shape::Shape;

pub fn component(path: &str, stack_role: &str, share: &str) -> Shape {
	Shape::Component {
		path: path.to_string(),
		stack_role: stack_role.to_string(),
		share: share.to_string(),
	}
}
