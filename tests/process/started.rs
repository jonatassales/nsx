//! The tools a test's runner was asked to start, each as its name and its
//! arguments

use crate::simulation::fake::programs::Programs;

pub fn started(programs: &Programs) -> Vec<String> {
	programs
		.started
		.borrow()
		.iter()
		.map(|program| format!("{} {}", program.name, program.arguments.join(" ")))
		.collect()
}
