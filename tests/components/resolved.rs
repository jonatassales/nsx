//! A Go component as the plan resolved it, at a path, a port and an image

use std::path::PathBuf;

use nsx::components::{
	component::Component, image::Image, manifest::Manifest, role::API, stack::GO,
};

pub fn resolved(path: &str, port: i64, image: Option<&str>) -> Component {
	Component {
		path: String::from(path),
		folder: PathBuf::from("/project").join(path),
		manifest: Manifest::GoMod,
		stack: GO,
		role: API,
		port,
		image: image.map(|reference| Image(String::from(reference))),
		cpu_millis: 250,
		memory_mib: 256,
	}
}
