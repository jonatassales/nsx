//! A session that keeps what it is given
//!
//! The session the tests play a command over: its lines and its title go to
//! a recorder, and it has no outside system to ask. A test replaces what its
//! case needs, such as the judge.

use std::path::PathBuf;
use std::time::Duration;

use nsx::command::session::Session;
use nsx::components::derived::Derived;
use nsx::console::offline::Offline;
use nsx::judgment::absent::Absent;
use nsx::process::local::Local;

use crate::simulation::fake::{
	calm::Calm, coder::CODER, forged::FORGED, host::HOST, moment::Moment,
	recorder::Recorder, silence::Silence,
};

pub fn session(directory: PathBuf, recorder: &Recorder) -> Session<'_> {
	Session {
		directory,
		sink: recorder,
		window: recorder,
		judge: &Absent,
		images: &Derived,
		console: &Offline,
		programs: &Local,
		answers: &Silence,
		agent: &CODER,
		clock: &Moment,
		cluster: &HOST,
		forge: &FORGED,
		interval: Duration::ZERO,
		beat: Duration::ZERO,
		interrupt: &Calm,
		environment: &|_name| None,
	}
}
