//! The runner that starts nothing: it keeps each program and answers as told
//!
//! Each program is answered with the next output a test scripted. Once the
//! script has run out, every program ends well and prints what the test
//! said.

mod run;

use std::cell::RefCell;
use std::collections::VecDeque;

use nsx::process::{output::Output, program::Program};

#[derive(Debug, Default)]
pub struct Programs {
	pub started: RefCell<Vec<Program>>,
	pub printed: String,
	pub scripted: RefCell<VecDeque<Output>>,
}
