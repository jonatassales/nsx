//! A link to a document a test placed in `.agents/`, by its file name

use std::path::PathBuf;

use nsx::request::{kind::KINDS, link::Link};

pub fn placed(name: &str) -> Link {
	let kind = KINDS
		.into_iter()
		.find(|kind| name.ends_with(kind.suffix))
		.unwrap();

	Link {
		kind,
		name: String::from(name),
		short: String::from("notes"),
		path: PathBuf::from(".agents").join(name),
	}
}
