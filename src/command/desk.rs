//! Where the plan's card data is kept and told
//!
//! The plan writes its card data to `plan.json` and tells the console each
//! time it does. This holds the two together, so that no write goes untold.
//!
//! Definition only: the `Desk` type: the one writer of `plan.json` and the
//! state of the reports to the console. Its function is the file in `desk/`.

pub mod record;

use crate::console::report::Report;
use crate::handoff::writer::Writer;

#[derive(Debug, Clone, PartialEq)]
pub struct Desk {
	pub writer: Writer,
	pub report: Report,
}
