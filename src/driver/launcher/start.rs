//! Starting the coding agent
//!
//! One function: it takes a call and the driver configuration, starts the
//! configured executable with its arguments filled, waits for it to end and
//! gives back what it printed and how it ended.
//!
//! ## Reads and writes
//!
//! - runs the `executable` of `driver.json`, with its `args` filled, in the
//!   project's root
//! - environment: gives the agent the minimal base, the `passEnv` variables,
//!   `LANGDRIFT_PROJECT_ROOT`, `LANGDRIFT_RUN`, `LANGDRIFT_TICKET`,
//!   `LANGDRIFT_STAGE`, `LANGDRIFT_SKILL`, `LANGDRIFT_MODEL`,
//!   `LANGDRIFT_EFFORT` and `LANGDRIFT_REPORT`
//!
//! ## Rules
//!
//! - The agent is started directly, with no shell. A placeholder is filled
//!   inside its argument and never splits it.
//! - Each configured argument is filled in one pass: a value that itself
//!   holds `{{effort}}`, such as a ticket's id, stays as it is.
//! - The placeholders are `{{prompt}}`, `{{model}}`, `{{effort}}`,
//!   `{{report}}`, `{{skill}}`, `{{stage}}`, `{{ticket}}` and `{{run}}`.
//!
//! Spec 0080.

use std::future::Future;
use std::pin::Pin;

use crate::driver::{
	agent::Agent, call::Call, config::Config, launcher::Launcher,
};
use crate::process::{
	environment::Environment, error::Error, output::Output, program::Program,
};

impl Agent for Launcher<'_> {
	fn start<'call>(
		&'call self,
		call: &'call Call,
		config: &'call Config,
	) -> Pin<Box<dyn Future<Output = Result<Output, Error>> + 'call>> {
		Box::pin(async move {
			let report = call.report.to_string_lossy().into_owned();
			let root = call.root.to_string_lossy().into_owned();
			let filled = [
				("{{prompt}}", call.prompt()),
				("{{model}}", call.model.clone()),
				("{{effort}}", call.effort.clone()),
				("{{report}}", report.clone()),
				("{{skill}}", call.skill.clone()),
				("{{stage}}", call.stage.clone()),
				("{{ticket}}", call.ticket.clone()),
				("{{run}}", call.run.clone()),
			];
			// One pass over the configured argument: a value that itself
			// looks like a placeholder is never filled again. A brace that
			// opens no placeholder is passed as written, and the next
			// character may still open one.
			let fill = |argument: &String| {
				let mut text = String::new();
				let mut rest = argument.as_str();

				while let Some(at) = rest.find('{') {
					let (before, opened) = rest.split_at(at);
					let known = filled
						.iter()
						.find(|(placeholder, _value)| opened.starts_with(placeholder));

					text.push_str(before);

					let Some((placeholder, value)) = known else {
						text.push('{');
						rest = &opened[1..];

						continue;
					};

					text.push_str(value);
					rest = &opened[placeholder.len()..];
				}

				text.push_str(rest);

				text
			};
			let arguments = config.args.iter().map(fill).collect();
			let passed = config
				.pass_env
				.iter()
				.map(String::as_str)
				.collect::<Vec<_>>();
			let mut environment = Environment::allow(&passed);
			let given = [
				("LANGDRIFT_PROJECT_ROOT", root),
				("LANGDRIFT_RUN", call.run.clone()),
				("LANGDRIFT_TICKET", call.ticket.clone()),
				("LANGDRIFT_STAGE", call.stage.clone()),
				("LANGDRIFT_SKILL", call.skill.clone()),
				("LANGDRIFT_MODEL", call.model.clone()),
				("LANGDRIFT_EFFORT", call.effort.clone()),
				("LANGDRIFT_REPORT", report),
			];

			for (name, value) in given {
				environment.variables.insert(String::from(name), value);
			}

			let program = Program {
				name: config.executable.clone(),
				arguments,
				environment,
				directory: call.root.clone(),
				input: Vec::new(),
			};

			self.programs.run(&program).await
		})
	}
}
