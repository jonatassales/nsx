//! Writing the plan's card data
//!
//! One function: it takes the project's root and what the folder that owns
//! a field does to the card data, applies it, and replaces `plan.json` with
//! the card data so far. It gives back nothing, or the failure to write.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/microruntimes/runtimes/<runtime id>/plan.json`
//!
//! ## Rules
//!
//! - The file is replaced atomically each time a line that fills a field
//!   prints, so that a receiver can fill the cards live.
//! - The file's shape is the one `contract` holds, so it is valid after
//!   every write and holds every field written before.
//! - The file never leaves the project: a runtime folder that is a symbolic
//!   link is refused.

use crate::contract::{cards::Cards, encode::encode};
use crate::files::{inside::inside, root::Root, write::write};
use crate::handoff::{error::Error, writer::Writer};

impl Writer {
	pub fn write(
		&mut self,
		root: &Root,
		fill: impl FnOnce(&mut Cards),
	) -> Result<(), Error> {
		fill(&mut self.cards);

		let file = inside(root, &self.file)?;
		let mut text = encode(&self.cards).map_err(|cause| Error::Unwritten {
			path: file.clone(),
			cause,
		})?;

		text.push('\n');

		Ok(write(&file, text.as_bytes())?)
	}
}
