//! Two resolved components, each with its port, its image and its share

use nsx::components::{component::Component, image::Image};

use crate::runtime::shared::shared;

pub fn imaged() -> Vec<Component> {
	vec![
		Component {
			port: 8080,
			image: Some(Image(String::from("notes/api:r1"))),
			..shared("services/api", 250, 256)
		},
		Component {
			port: 8081,
			image: Some(Image(String::from("notes/jobs:r1"))),
			..shared("services/jobs", 300, 512)
		},
	]
}
