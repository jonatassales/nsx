//! Reading the PRD
//!
//! One function: it takes the project's root and the PRD's link, and gives
//! back the PRD with its use case.
//!
//! ## Reads and writes
//!
//! - reads the linked file in the client's artifact folder
//!
//! ## Rules
//!
//! - The `**Use case:**` line gives the use case.
//! - A missing, refused or unreadable link, or a PRD without that line, stops
//!   the plan (the stop rule). No link inside the PRD is followed.
//!
//! ## Open
//!
//! - Whether LangDrift imposes this field line on the client's documents is
//!   the owner's decision.
//!
//! Spec 0078.

use crate::files::{
	root::Root,
	text::{DOCUMENT, text},
};
use crate::request::{error::Error, field::field, link::Link, prd::Prd};

impl Prd {
	pub fn read(root: &Root, link: &Link) -> Result<Self, Error> {
		let written = text(root, &link.path, DOCUMENT)?;
		let use_case = field(&written, "Use case").ok_or_else(|| Error::Field {
			path: link.path.clone(),
			field: "Use case",
		})?;

		Ok(Self {
			link: link.clone(),
			use_case: String::from(use_case),
		})
	}
}
