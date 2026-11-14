//! Every program is missing

use std::future::Future;
use std::pin::Pin;

use nsx::process::{
	error::Error, output::Output, program::Program, runner::Runner,
};

use crate::simulation::fake::bare::Bare;

impl Runner for Bare {
	fn run<'call>(
		&'call self,
		program: &'call Program,
	) -> Pin<Box<dyn Future<Output = Result<Output, Error>> + 'call>> {
		Box::pin(async move {
			Err(Error::NotRun {
				program: program.name.clone(),
				cause: std::io::Error::from(std::io::ErrorKind::NotFound),
			})
		})
	}
}
