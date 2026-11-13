//! A program to run
//!
//! What it takes to start one program: its name, each argument whole, the
//! environment it will see, the folder it starts in and what it is given to
//! read.
//!
//! Definition only: the `Program` type, which carries the name, the
//! arguments, the environment, the folder and the input. Its function is the
//! file in `program/`.
//!
//! ## Rules
//!
//! - The name is looked up on the environment's `PATH`, never a shell line.
//! - The arguments are never split or joined.
//! - The input is what the program reads on its standard input. With none,
//!   the program reads nothing.
//!
//! Shared: used by `driver`, `local` and `delivery`.

pub mod tool;

use std::path::PathBuf;

use crate::process::environment::Environment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
	pub name: String,
	pub arguments: Vec<String>,
	pub environment: Environment,
	pub directory: PathBuf,
	pub input: Vec<u8>,
}
