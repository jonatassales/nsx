//! The text of every line the six goldens hold

use std::path::Path;

pub fn goldens() -> Vec<String> {
	let goldens =
		Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/simulation/golden");

	std::fs::read_dir(goldens)
		.unwrap()
		.map(|entry| std::fs::read_to_string(entry.unwrap().path()).unwrap())
		.flat_map(|golden| {
			golden
				.lines()
				.map(|row| row.splitn(3, '\t').nth(2).unwrap().to_string())
				.collect::<Vec<_>>()
		})
		.collect()
}
