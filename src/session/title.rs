//! The terminal's window title
//!
//! The window names the tool and the project: the command's name, an em dash
//! between single spaces, and the name of the project's folder, the one that
//! holds `.langdrift/`.
//!
//! Definition only: the `Title` type, which carries the name of the
//! project's folder. Its function is the file in `title/`.
//!
//! ## Simulation
//!
//! - The window title: `nsx — project`
//!
//! ## Rules
//!
//! - The plan and the run set it when they start, with the standard terminal
//!   title sequence, and only when standard output is a terminal.

pub mod of;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title {
	pub project: String,
}
