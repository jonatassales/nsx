//! The forge that keeps what it is asked and answers a script
//!
//! Each operation is kept, in order. The checks of the commit it pushed are
//! answered with the next listing a test scripted, and the last one is
//! repeated once the script has run out. Asked about another commit, it
//! has none to give.

mod forge;

use std::cell::RefCell;
use std::collections::VecDeque;

use nsx::checks::check::Check;

#[derive(Debug, Default)]
pub struct Hub {
	pub listings: RefCell<VecDeque<Vec<Check>>>,
	pub asked: RefCell<Vec<String>>,
	pub commit: String,
	pub opened: (u64, String),
}
