//! Reading the expected tokens
//!
//! One function: it takes the project's root and the paths under
//! `.langdrift/`, reads the project's forecast and gives it back.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/finops/forecasts.json`
//!
//! ## Rules
//!
//! - A missing or unreadable forecast stops the command before the line that
//!   would need it (the stop rule). No refusal line is printed.
//! - The plan needs it for the spend table, so it is read before the plan's
//!   first line.
//! - A field the forecast does not define is refused, and so is a count that
//!   is negative or not whole.
//!
//! Spec 0079.

use crate::contract::decode::decode;
use crate::files::{
	layout::Layout,
	root::Root,
	text::{DOCUMENT, text},
};
use crate::money::{error::Error, forecast::Forecast};

impl Forecast {
	pub fn load(root: &Root, layout: &Layout) -> Result<Self, Error> {
		let written = text(root, &layout.forecasts, DOCUMENT)?;

		decode(&written).map_err(|cause| Error::Forecasts {
			path: layout.forecasts.clone(),
			cause,
		})
	}
}
