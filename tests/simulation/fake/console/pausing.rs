//! The request of a console told to pause before a stage

use nsx::contract::{
	asked::Asked, report::Report, requested::Requested, status::Status,
};

use crate::simulation::fake::console::Console;

impl Console {
	pub fn pausing(&self, report: &Report) -> Option<Requested> {
		let stage = self.pause_before?;
		let request = |state: Asked| Requested {
			state,
			intent: None,
		};

		if report.status == Status::Held {
			self.holds.set(self.holds.get() + 1);

			let played = self
				.plays_after
				.is_some_and(|holds| self.holds.get() >= holds);

			return played.then(|| request(Asked::Play));
		}

		let steps = report
			.run
			.iter()
			.flat_map(|run| run.stages.iter().flatten())
			.collect::<Vec<_>>();
		let at = steps.iter().position(|step| step.id == stage);
		let before = at.is_some_and(|at| {
			let previous = at.checked_sub(1).map(|earlier| steps[earlier]);
			let step = steps[at];

			previous.is_some_and(|previous| previous.state == "done")
				&& step.state != "active"
				&& step.state != "done"
				&& step.report.is_none()
		});
		let pausing =
			report.status == Status::Workflow && self.holds.get() == 0 && before;

		pausing.then(|| request(Asked::Pause))
	}
}
