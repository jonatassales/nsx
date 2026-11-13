//! The single writer of the plan's card data
//!
//! `plan.json` holds every field of the Ticket, Analysis and Plan cards and
//! of the phase-1 Workflow card. The card is the specification of the data.
//! Each folder that owns a field hands it to this writer as its line prints,
//! and no folder writes the file directly.
//!
//! Definition only: the `Writer` type, which holds the file's path and the
//! plan's card data as it fills. Its function is the file in `writer/`.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/microruntimes/runtimes/<runtime id>/plan.json`
//!
//! ## Rules
//!
//! - A field is absent until its line has printed. Two exceptions are
//!   written first, before the plan's first line: the ticket's header and
//!   the `workflow` section.
//! - Values are units, never formatted text: millicores and MiB as integers,
//!   money as integer cents, similarities as numbers with two decimals.
//! - What is unknown is `null`, never zero.
//! - The run never touches `plan.json`.
//! - A plan that stops leaves `plan.json` as far as it got.
//!
//! ## Open
//!
//! - The Plan card after the run. It still reads "pending team
//!   authorization" while the run's opening comment says "authorized by the
//!   team". nsx follows the simulation until the owner decides.

pub mod write;

use std::path::PathBuf;

use crate::contract::cards::Cards;

#[derive(Debug, Clone, PartialEq)]
pub struct Writer {
	pub file: PathBuf,
	pub cards: Cards,
}
