//! The console that keeps every report and answers one runtime id
//!
//! Each answer carries the next request a test scripted, when there is one.
//! Without one, a console told to pause before a stage asks for a pause once
//! the stage before it is done and its own work has not begun, and for a
//! play once the run has reported `held` as often as the test says. A test
//! can make the console down, down from one report on, or forget one
//! runtime id, and can have a file rewritten as one report arrives.

mod pausing;
mod report;

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::path::PathBuf;

use nsx::contract::{report::Report, requested::Requested};

pub const RUNTIME: &str = "019a0000-0000-7000-8000-00000000cafe";

#[derive(Debug, Default)]
pub struct Console {
	pub reports: RefCell<Vec<Report>>,
	pub down: bool,
	pub silent_from: Option<usize>,
	pub forgets: Option<String>,
	pub requests: RefCell<VecDeque<Option<Requested>>>,
	pub pause_before: Option<&'static str>,
	pub plays_after: Option<u32>,
	pub holds: Cell<u32>,
	pub rewrite: RefCell<Option<(usize, PathBuf, String)>>,
}
