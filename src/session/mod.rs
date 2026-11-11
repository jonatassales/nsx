//! The frame of the session
//!
//! The session is what a person sees in the terminal. This folder fixes what
//! every other folder relies on to print it: the kinds of line, the layout
//! of a line, the sink a line goes through, the window title, and the rule
//! that ends a command on a failure the simulation does not show.
//!
//! - `Line`: one line, with its id, its kind and its shape
//! - `Shape`: the six ways a line is laid out
//! - `Kind`: the four kinds: comment, output, success, warning
//! - `Sink`: the trait every line is printed through
//! - `Terminal`: the sink that writes to the terminal
//! - `Title`: the window title
//! - `Window`: the trait the title is set through
//! - `Stop`: the stop rule: one sentence on stderr, then the exit
//! - `Interrupt`: the person's interrupt, as a trait
//! - `Signal`: the interrupt as the system sends it
//!
//! ## Rules
//!
//! - Every actor line starts with `console › `. The prefix is part of the
//!   output, not a claim that the console reasons (ADR 0005).
//! - Standard output is always a prefix of the simulation's transcript.
//! - Nothing the simulation does not show is printed.
//!
//! Spec 0077.

pub mod interrupt;
pub mod kind;
pub mod line;
pub mod shape;
pub mod signal;
pub mod sink;
pub mod stop;
pub mod terminal;
pub mod title;
pub mod window;
