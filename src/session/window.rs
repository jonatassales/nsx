//! The window a session shows in
//!
//! The simulation's terminal has a title bar, and the plan and the run name
//! the tool and the project there. A sink that is not a terminal has no
//! window, and a test's fake only remembers the title it was given.
//!
//! Definition only: the trait `Window`, which takes the window's title, so
//! that tests can replace it with a fake.
//!
//! ## Rules
//!
//! - The title is never part of standard output's lines: a piped session
//!   holds no trace of it.
//!
//! Spec 0077.

use crate::session::{stop::Stop, title::Title};

pub trait Window {
	fn title(&self, title: &Title) -> Result<(), Stop>;
}
