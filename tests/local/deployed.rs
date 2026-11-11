//! A Deployment of a quarter core and 256 MiB, by its name

use nsx::local::deployment::Deployment;

pub fn deployed(name: &str) -> Deployment {
	Deployment {
		name: String::from(name),
		image: format!("notes/{name}:r1"),
		port: 8080,
		cpu_millis: 250,
		memory_mib: 256,
		replicas: 1,
	}
}
