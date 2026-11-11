//! The sink every line is printed through
//!
//! No folder writes to the terminal itself. Each hands its line to the sink:
//! the line's id, its kind and its data. The golden test reads each line's
//! kind from the sink and compares it with the simulation's.
//!
//! Definition only: the trait `Sink`, which takes one line at a time, so
//! that tests can replace it with a fake. Printing gives back a stop when
//! the line cannot be written.
//!
//! ## Rules
//!
//! - Lines print when their work is done. No line is delayed to imitate the
//!   simulation's pacing.
//! - Nothing the simulation does not show is printed.
//!
//! Spec 0077.

use crate::session::{line::Line, stop::Stop};

pub trait Sink {
	fn print(&self, line: &Line) -> Result<(), Stop>;
}
