//! Running a program
//!
//! The one way nsx starts another program and waits for it to end. The coding
//! agent, the forge's tools and the cluster's tools are all reached through
//! it.
//!
//! Definition only: the trait `Runner`, which takes a program, with its
//! arguments, its environment and its folder, and gives back its output, so
//! that tests can replace it with a fake. Its function is the file in
//! `runner/`.
//!
//! Shared: used by `driver`, `local` and `delivery`.

pub mod finish;

use std::future::Future;
use std::pin::Pin;

use crate::process::{error::Error, output::Output, program::Program};

pub trait Runner {
	fn run<'call>(
		&'call self,
		program: &'call Program,
	) -> Pin<Box<dyn Future<Output = Result<Output, Error>> + 'call>>;
}
