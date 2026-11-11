//! A run played in a planned project, with the coding agent and the answers
//! a test gives it

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use nsx::command::run::Run;
use nsx::session::stop::Stop;

use crate::simulation::fake::{
	answers::Answers, coder::Coder, recorder::Recorder, session::session,
};

pub async fn played(
	project: &Path,
	coder: Coder,
	answers: usize,
) -> (Result<(), Stop>, Vec<String>) {
	let recorder = Recorder::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); answers].into()),
	};
	let mut session = session(PathBuf::from(project), &recorder);

	session.answers = &answers;
	session.agent = &coder;

	let ended = Run.execute(&mut session).await;
	let printed = recorder
		.lines
		.borrow()
		.iter()
		.map(|line| line.id.clone())
		.collect();

	(ended, printed)
}
