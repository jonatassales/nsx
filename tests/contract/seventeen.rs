//! One component more than a fence holds
//!
//! Each on its own path, port and image, with a share of one millicore and
//! one MiB, so that the count is the only rule they break.

use nsx::contract::component::Component;

pub fn seventeen(template: &Component) -> Vec<Component> {
	(0..17)
		.map(|index| Component {
			path: format!("services/api-{index}"),
			port: 9000 + index,
			image: format!("book-a-demo-runtime/api-{index}:r1"),
			cpu_millis: 1,
			memory_mib: 1,
			..template.clone()
		})
		.collect()
}
