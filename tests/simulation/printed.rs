//! What nsx printed, as a golden would freeze it

use nsx::session::kind::Kind;

use crate::simulation::{fake::recorder::Recorder, frozen::Frozen};

pub fn printed(recorder: &Recorder) -> Vec<Frozen> {
	recorder
		.lines
		.borrow()
		.iter()
		.map(|line| {
			let kind = match line.kind {
				Kind::Comment => "comment",
				Kind::Output => "output",
				Kind::Success => "success",
				Kind::Warning => "warning",
			};

			Frozen {
				id: line.id.clone(),
				kind: String::from(kind),
				text: line.render(),
			}
		})
		.collect()
}
