//! A tool pinned to the SHA-256 of some bytes, as a test fetches it

use nsx::local::tool::Tool;
use sha2::{Digest, Sha256};

pub fn pinned(bytes: &[u8]) -> Tool {
	let spelled: String = Sha256::digest(bytes)
		.iter()
		.map(|byte| format!("{byte:02x}"))
		.collect();

	Tool {
		name: "kind",
		url: String::from("https://example.invalid/kind"),
		sha256: spelled,
	}
}
