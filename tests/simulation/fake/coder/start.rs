//! One stage's work: the report on disk, and the usage on standard output

use std::future::Future;
use std::pin::Pin;

use nsx::driver::{agent::Agent, call::Call, config::Config};
use nsx::process::{error::Error, output::Output};

use crate::contract::sample::sample;
use crate::simulation::fake::coder::Coder;

impl Agent for Coder {
	fn start<'call>(
		&'call self,
		call: &'call Call,
		_config: &'call Config,
	) -> Pin<Box<dyn Future<Output = Result<Output, Error>> + 'call>> {
		Box::pin(async move {
			let stage = call.stage.as_str();

			if self.stalling == Some(stage) {
				std::future::pending::<()>().await;
			}

			let report = call.root.join(&call.report);
			let figures = sample(&format!("stages/{stage}.report.sample.json"));
			let usage = if self.unmeasured == Some(stage) {
				String::from("{}")
			} else {
				format!(
					r#"{{"usage": {{"input_tokens": {}, "output_tokens": {}}}}}"#,
					self.input, self.output
				)
			};
			let status = if self.failing == Some(stage) { 2 } else { 0 };
			let blocked = self
				.blocking
				.filter(|(during, _path)| *during == stage)
				.map(|(_during, path)| call.root.join(path));

			if let Some(blocked) = blocked {
				std::fs::create_dir_all(blocked).unwrap();
			}

			if self.silent != Some(stage) {
				std::fs::create_dir_all(report.parent().unwrap()).unwrap();
				std::fs::write(&report, figures).unwrap();
			}

			Ok(Output {
				status: Some(status),
				printed: usage.into_bytes(),
				complained: Vec::new(),
			})
		})
	}
}
