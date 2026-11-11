//! A shell script as a program to run

use nsx::process::{environment::Environment, program::Program};

pub fn shell(script: &str, environment: Environment) -> Program {
	Program {
		name: String::from("/bin/sh"),
		arguments: vec![String::from("-c"), script.to_string()],
		environment,
		directory: std::env::temp_dir(),
		input: Vec::new(),
	}
}
