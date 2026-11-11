//! The implement call of a run of a notes runtime

use std::path::{Path, PathBuf};

use nsx::driver::call::Call;

pub const REPORT: &str =
	".langdrift/agent/state/runs/notes-0001/stages/implement.report.json";

pub fn called(root: &Path) -> Call {
	Call {
		root: PathBuf::from(root),
		run: String::from("notes-0001"),
		ticket: String::from("0007-TICKET-notes"),
		stage: String::from("implement"),
		skill: String::from("implement · tdd"),
		model: String::from("claude-sonnet-5-5"),
		effort: String::from("medium"),
		report: PathBuf::from(REPORT),
	}
}
