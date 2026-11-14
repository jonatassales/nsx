//! A project of its own, planned by the real nsx
//!
//! A copy of the fixture project whose ticket carries the generated number
//! and slug, connected to the console the environment names, with what
//! `nsx agent plan` printed and how it ended.

pub mod nsx;
pub mod play;

use std::process::Output;

use tempfile::TempDir;

pub struct Played {
	pub project: TempDir,
	pub plan: Output,
}
