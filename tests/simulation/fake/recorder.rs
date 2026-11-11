//! The sink that keeps what it is given
//!
//! A session that is read back instead of watched: each line is kept, in the
//! order it was printed, and so is the window's title. The golden test plays
//! a command over one and compares what it kept with the simulation.
//!
//! Definition only: the `Recorder` type, which carries the lines printed so
//! far and the last title set. Its functions are the files in `recorder/`.
//!
//! ## Rules
//!
//! - It writes nothing anywhere: standard output stays empty.
//!
//! Spec 0077.

mod print;
mod title;

use std::cell::RefCell;

use nsx::session::{line::Line, title::Title};

#[derive(Debug, Default)]
pub struct Recorder {
	pub lines: RefCell<Vec<Line>>,
	pub title: RefCell<Option<Title>>,
}
