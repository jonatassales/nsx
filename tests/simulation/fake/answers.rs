//! The person who answers each wait with the next line a test gave

mod line;

use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Answers {
	pub lines: RefCell<VecDeque<String>>,
}
