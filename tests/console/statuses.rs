//! The statuses a console was told, in order

use nsx::contract::status::Status;

use crate::simulation::fake::console::Console;

pub fn statuses(console: &Console) -> Vec<Status> {
	console
		.reports
		.borrow()
		.iter()
		.map(|report| report.status)
		.collect()
}
