//! The layout of a line
//!
//! Labels and values line up down the terminal as they do in the simulation,
//! because every line takes one of six shapes.
//!
//! One function: it gives back a line's text in the line format: one of the
//! six shapes below, with its columns padded.
//!
//! ## Rules
//!
//! - Comment: `# `, then the text.
//! - Narrative actor line: `console › `, then the text.
//! - Labeled actor line: `console › `, the label padded to 10 columns, one
//!   space, the value.
//! - Result line: `✓ `, the label padded to 10 columns, one space, the value.
//! - Status line: the label padded to 10 columns, one space, the value.
//! - Component row: `✓ `, the path padded to 19 columns, three spaces,
//!   `<stack> · <role>` padded to 15 columns, one space,
//!   `<cpu>m · <memory> MiB`.
//! - A label of 10 columns or more is followed by exactly one space
//!   (`to-tickets`, `code-review`).
//! - Columns count characters: `›`, `·`, `✓` and the em dash count one each.

use crate::session::{line::Line, shape::Shape};

const LABEL_COLUMNS: usize = 10;
const PATH_COLUMNS: usize = 19;
const STACK_ROLE_COLUMNS: usize = 15;

impl Line {
	pub fn render(&self) -> String {
		let text = match &self.shape {
			Shape::Comment {
				text,
			} => format!("# {text}"),
			Shape::Narrative {
				text,
			} => format!("console › {text}"),
			Shape::Labeled {
				label,
				value,
			} => format!("console › {label:<LABEL_COLUMNS$} {value}"),
			Shape::Result {
				label,
				value,
			} => format!("✓ {label:<LABEL_COLUMNS$} {value}"),
			Shape::Status {
				label,
				value,
			} => format!("{label:<LABEL_COLUMNS$} {value}"),
			Shape::Component {
				path,
				stack_role,
				share,
			} => format!(
				"✓ {path:<PATH_COLUMNS$}   {stack_role:<STACK_ROLE_COLUMNS$} {share}"
			),
		};

		text.trim_end_matches(' ').to_string()
	}
}
