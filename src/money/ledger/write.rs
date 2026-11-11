//! Writing the run's money record
//!
//! One function: it takes the project's root and the paths under
//! `.langdrift/`, and writes the run's money entry as it stands to the
//! ledger, under the run's id.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/finops/ledger/<run>.ledger.json`
//!
//! ## Rules
//!
//! - `<run>` is the run's id, which `evidence/` gives when the run starts.
//! - The entry is opened at the run's `estimate` line. It is written again
//!   when the gate is decided, when the observed spend is priced and when the
//!   release is known.
//! - A paused run keeps its entry, with the gate paused and nothing observed.
//! - The record holds amounts and tokens, and no secret.
//!
//! Spec 0080.

use crate::contract::encode::encode;
use crate::files::{inside::inside, layout::Layout, root::Root, write::write};
use crate::money::{error::Error, ledger::Ledger};

impl Ledger {
	pub fn write(&self, root: &Root, layout: &Layout) -> Result<(), Error> {
		let name = format!("{}.ledger.json", self.run);
		let file = inside(root, &layout.ledger.join(name))?;
		let mut text = encode(self).map_err(|cause| Error::Unwritten {
			path: file.clone(),
			cause,
		})?;

		text.push('\n');

		Ok(write(&file, text.as_bytes())?)
	}
}
