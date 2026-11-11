//! The lines of one golden: `id`, kind and text, a tab between them

use std::path::Path;

use crate::simulation::frozen::Frozen;

pub fn golden(name: &str) -> Vec<Frozen> {
	let path = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("tests/simulation/golden")
		.join(format!("{name}.golden"));

	std::fs::read_to_string(path)
		.unwrap()
		.lines()
		.map(|line| {
			let mut fields = line.splitn(3, '\t');

			Frozen {
				id: String::from(fields.next().unwrap()),
				kind: String::from(fields.next().unwrap()),
				text: String::from(fields.next().unwrap()),
			}
		})
		.collect()
}
