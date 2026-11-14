//! The five kinds of line
//!
//! Every printed line carries the kind the simulation gives it: comment,
//! output, success, warning or error. The kind is the second column of the
//! goldens, and it sets the line's tone in a terminal.
//!
//! Definition only: the `Kind` type, with exactly five kinds: comment,
//! output, success, warning and error.
//!
//! ## Rules
//!
//! - In a terminal a comment is dimmed, a success line is green, a warning is
//!   yellow, an error line is red, and an output line keeps the terminal's
//!   default color.
//! - `# …` lines are comments and `✓ …` lines are success. `limits` is a
//!   warning. The opened gate is a success line although it has no mark,
//!   and the paused gate is a warning. `✗ …` lines are errors. Every other
//!   line is output.
//! - The simulation has a kind for the typed command. It is the person's
//!   typing and is never printed, so it is not one of the five.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
	Comment,
	Output,
	Success,
	Warning,
	Error,
}
