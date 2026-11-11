//! Every file under a folder, by its path from that folder, with its bytes

use std::collections::BTreeMap;
use std::path::Path;

pub fn tree(folder: &Path) -> BTreeMap<String, Vec<u8>> {
	let mut files = BTreeMap::new();

	for entry in std::fs::read_dir(folder).unwrap() {
		let entry = entry.unwrap();
		let name = entry.file_name().to_string_lossy().into_owned();

		if !entry.file_type().unwrap().is_dir() {
			files.insert(name, std::fs::read(entry.path()).unwrap());

			continue;
		}

		for (inner, bytes) in tree(&entry.path()) {
			files.insert(format!("{name}/{inner}"), bytes);
		}
	}

	files
}
