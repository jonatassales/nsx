//! Starting a program and waiting for it
//!
//! One function: it takes a program, its arguments and its environment, starts
//! the program, gives it its input, waits for it to end and gives back its
//! output.
//!
//! ## Reads and writes
//!
//! - runs the program it is given
//!
//! ## Rules
//!
//! - The program is started directly, never through a shell, and each
//!   argument reaches it whole (spec 0058).
//! - The program sees only the variables of the environment it is given,
//!   never nsx's own (ADR 0002).
//! - A program with no input reads nothing: its standard input is closed.
//! - It prints nothing. A program that fails is its caller's stop.
//!
//! ## Open
//!
//! - What an interrupt does to a program still running. Ctrl-C is a stop
//!   (spec 0048), and no spec says how the program is ended. Until one
//!   does, a program is killed when the command that waits for it stops.
//! - Whether a program has a time limit. Spec 0058 leaves it open for the
//!   coding agent.
//!
//! Shared: used by `driver`, `local` and `delivery`.

use std::future::Future;
use std::pin::Pin;
use std::process::Stdio;

use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use crate::process::{
	error::Error, local::Local, output::Output, program::Program, runner::Runner,
};

impl Runner for Local {
	fn run<'call>(
		&'call self,
		program: &'call Program,
	) -> Pin<Box<dyn Future<Output = Result<Output, Error>> + 'call>> {
		Box::pin(async move {
			let unrun = |cause: std::io::Error| Error::NotRun {
				program: program.name.clone(),
				cause,
			};
			let reading = if program.input.is_empty() {
				Stdio::null()
			} else {
				Stdio::piped()
			};
			let mut started = Command::new(&program.name)
				.args(&program.arguments)
				.env_clear()
				.envs(&program.environment.variables)
				.current_dir(&program.directory)
				.stdin(reading)
				.stdout(Stdio::piped())
				.stderr(Stdio::piped())
				.kill_on_drop(true)
				.spawn()
				.map_err(unrun)?;

			// The input is written whole, then closed, so that the program
			// sees its end.
			if let Some(mut given) = started.stdin.take() {
				given.write_all(&program.input).await.map_err(unrun)?;

				given.shutdown().await.map_err(unrun)?;
			}

			let ended = started.wait_with_output().await.map_err(unrun)?;

			Ok(Output {
				status: ended.status.code(),
				printed: ended.stdout,
				complained: ended.stderr,
			})
		})
	}
}
