//! Keeping the program, and answering what the test said it prints

use std::future::Future;
use std::pin::Pin;

use nsx::process::{
	error::Error, output::Output, program::Program, runner::Runner,
};

use crate::simulation::fake::programs::Programs;

impl Runner for Programs {
	fn run<'call>(
		&'call self,
		program: &'call Program,
	) -> Pin<Box<dyn Future<Output = Result<Output, Error>> + 'call>> {
		Box::pin(async move {
			self.started.borrow_mut().push(program.clone());

			if let Some(scripted) = self.scripted.borrow_mut().pop_front() {
				return Ok(scripted);
			}

			Ok(Output {
				status: Some(0),
				printed: self.printed.clone().into_bytes(),
				complained: Vec::new(),
			})
		})
	}
}
