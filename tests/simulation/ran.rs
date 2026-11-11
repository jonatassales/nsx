//! What `nsx agent run` prints in a planned project, for the choice a test
//! names, with a person who answers yes at every wait

use std::cell::RefCell;
use std::path::PathBuf;

use nsx::command::run::Run;

use crate::simulation::{
	authorized::authorized,
	fake::{
		answers::Answers,
		coder::{CODER, Coder},
		recorder::Recorder,
		session::session,
	},
	frozen::Frozen,
	printed::printed,
};

pub async fn ran(model: &str, effort: &str, checked: &[&str]) -> Vec<Frozen> {
	let project = authorized(model, effort, checked).await;
	let recorder = Recorder::default();
	let answers = Answers {
		lines: RefCell::new(vec![String::from("yes"); 8].into()),
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);
	// What the five calls used, as the simulation's figures need it: Haiku's
	// run observes $0.27, the others' $9.80.
	let coder = match model {
		"claude-haiku-5-5" => Coder {
			input: 420_000,
			output: 24_000,
			..CODER
		},
		_ => CODER,
	};

	session.answers = &answers;
	session.agent = &coder;

	Run.execute(&mut session).await.unwrap();

	printed(&recorder)
}
