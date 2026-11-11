//! A planned project whose intent is still proposed, with the choice the plan
//! left in it

use nsx::contract::{decode::decode, encode::encode, intent::Intent};
use tempfile::TempDir;

use crate::simulation::authorized::{INTENT, authorized};

pub async fn proposed() -> (TempDir, Intent) {
	let project = authorized("claude-sonnet-5-5", "medium", &["review"]).await;
	let file = project.path().join(INTENT);
	let mut intent =
		decode::<Intent>(&std::fs::read_to_string(&file).unwrap()).unwrap();

	intent.status = String::from("proposed");

	std::fs::write(&file, encode(&intent).unwrap()).unwrap();

	(project, intent)
}
