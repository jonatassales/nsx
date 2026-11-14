//! A web that holds one file and counts how often it is asked

mod get;

use std::cell::Cell;

pub struct Shelf {
	pub bytes: Vec<u8>,
	pub asked: Cell<u32>,
}
