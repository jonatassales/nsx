//! A component of the fence, at the path a test gives it

use nsx::contract::component::Component;

pub fn component(path: &str) -> Component {
	Component {
		path: String::from(path),
		stack: String::from("Go"),
		role: String::from("api"),
		port: 8081,
		image: String::from("book-a-demo-runtime/signup-api:r1"),
		cpu_millis: 250,
		memory_mib: 256,
		replicas: 1,
	}
}
