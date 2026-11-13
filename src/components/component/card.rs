//! A component's row on the Plan card
//!
//! One function: it fills the component's row in the COMPONENTS of the plan's
//! cards: its manifest, stack, role, port and proposed share.
//!
//! ## Simulation
//!
//! - Card: Plan · COMPONENTS rows, each set by its own row line
//!
//! ## Rules
//!
//! - The row is the placeholder the `resolve` line left for the component's
//!   path, the first one not filled yet. A component with no placeholder gets
//!   a new row.
//! - This function writes no file: the one writer in `handoff/` does.

use crate::components::component::Component;
use crate::contract::{cards::Cards, resolved::Resolved};

impl Component {
	pub fn card(&self, cards: &mut Cards) {
		let rows = cards.components().rows.get_or_insert_default();
		let awaited = |row: &Resolved| row.path == self.path && row.stack.is_none();
		let at = rows.iter().position(awaited).unwrap_or(rows.len());

		if at == rows.len() {
			rows.push(Resolved {
				path: self.path.clone(),
				..Resolved::default()
			});
		}

		let row = &mut rows[at];

		row.manifest = Some(String::from(self.manifest.file()));
		row.stack = Some(String::from(self.stack.name));
		row.role = Some(String::from(self.role.name));
		row.port = Some(self.port);
		row.cpu_millis = Some(self.cpu_millis);
		row.memory_mib = Some(self.memory_mib);
	}
}
