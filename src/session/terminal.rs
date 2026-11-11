//! The sink that writes to the terminal
//!
//! The session a person reads: each line goes to standard output, with its
//! tone when standard output is a terminal, and as plain text when it is
//! piped.
//!
//! Definition only: the `Terminal` type, the sink over standard output, which
//! knows whether that output is a terminal. Its functions are the files in
//! `terminal/`.
//!
//! ## Rules
//!
//! - Piped output carries no escape sequence: a file holds exactly the
//!   session's lines.
//! - Standard error is not the session.
//!
//! Spec 0077.

pub mod print;
pub mod title;

#[derive(Debug, Clone, Copy)]
pub struct Terminal {
	pub toned: bool,
}
