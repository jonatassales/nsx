//! A project `nsx init` was run in, and what the command printed

use std::path::{Path, PathBuf};

use nsx::command::init::Init;

use crate::simulation::fake::recorder::Recorder;
use crate::simulation::fake::session::session;

pub async fn laid(project: &Path) -> Recorder {
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project), &recorder);

	Init.execute(&mut session).await.unwrap();

	recorder
}
