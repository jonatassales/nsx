//! Reading the guardrail document
//!
//! One function: it takes the project's root and the guardrail link the
//! ticket carries, and gives back the guardrail with its criteria, in table
//! order.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/harness/guardrails/<number>-<name>.gr.md`
//!
//! ## Rules
//!
//! - A guardrail document holds one criteria table whose columns are `ID`, the
//!   rule and `Checked at` (`plan`, `run`, or `plan, run`).
//! - No guardrail link, an unreadable guardrail, a guardrail without a criteria
//!   table or a row lacking `Checked at` stops the plan before the guardrail's
//!   line, the one that opens with `reads` (the stop rule). A plan never
//!   claims rules it did not read.
//! - The table is the document's first one, with its row of dashes under the
//!   header: a table without that row is no table, so no criterion is ever
//!   passed over. The rule is the column that is neither `ID` nor
//!   `Checked at`.
//!
//! ## Open
//!
//! - Whether a `Checked at` column is the way a guardrail says which step a
//!   criterion governs. The column is provisional: if the `reads` line's
//!   omission of FIN-03 is a slip, the column is dropped.

use crate::files::{
	root::Root,
	text::{DOCUMENT, text},
};
use crate::harness::{
	criterion::Criterion, error::Error, guardrail::Guardrail,
};
use crate::request::link::Link;

impl Guardrail {
	pub fn load(root: &Root, link: &Link) -> Result<Self, Error> {
		// The cells of one table row, without the bars at its ends.
		fn cells(row: &str) -> Vec<&str> {
			row
				.trim()
				.trim_matches('|')
				.split('|')
				.map(str::trim)
				.collect()
		}

		let written = text(root, &link.path, DOCUMENT)?;
		let no_table = || Error::NoTable {
			path: link.path.clone(),
		};
		let mut rows = written
			.lines()
			.skip_while(|line| !line.trim_start().starts_with('|'))
			.take_while(|line| line.trim_start().starts_with('|'))
			.map(cells);

		let header = rows.next().ok_or_else(no_table)?;
		let ruler = rows.next().ok_or_else(no_table)?;

		// A table has a row of dashes under its header. Without one the
		// second row is a criterion, and passing over it would lose a rule.
		let ruled = ruler.iter().all(|cell| {
			let dashes = cell.trim_matches(':');

			!dashes.is_empty() && dashes.chars().all(|dash| dash == '-')
		});

		if !ruled {
			return Err(no_table());
		}

		let column = |name: &str| header.iter().position(|cell| *cell == name);
		let id = column("ID").ok_or_else(no_table)?;
		let checked = column("Checked at").ok_or_else(no_table)?;
		let rule = (0..header.len())
			.find(|index| *index != id && *index != checked)
			.ok_or_else(no_table)?;

		let criteria = rows
			.map(|row| {
				let cell = |index: usize| row.get(index).copied().unwrap_or_default();
				let steps = cell(checked).split(',').map(str::trim).collect::<Vec<_>>();
				let known = !cell(id).is_empty()
					&& steps.iter().all(|step| matches!(*step, "plan" | "run"));

				if !known {
					return Err(Error::Row {
						path: link.path.clone(),
						row: row.join(" | "),
					});
				}

				Ok(Criterion {
					id: String::from(cell(id)),
					rule: String::from(cell(rule)),
					plan: steps.contains(&"plan"),
					run: steps.contains(&"run"),
				})
			})
			.collect::<Result<Vec<_>, _>>()?;

		if criteria.is_empty() {
			return Err(no_table());
		}

		Ok(Self {
			path: link.path.clone(),
			name: link.name.clone(),
			short: link.short.clone(),
			criteria,
		})
	}
}
