//! The few variables a program is given
//!
//! The whole environment of a program nsx starts. nsx's own environment can
//! hold a secret, so no program inherits it: each one gets a minimal base,
//! plus what its family of tools needs, plus what its caller adds.
//!
//! Definition only: the `Environment` type: the variables a program will see,
//! by name. Its function is the file in `environment/`.
//!
//! ## Rules
//!
//! - The base is `PATH`, `HOME`, `TMPDIR` and the locale.
//! - The coding agent also gets the `LANGDRIFT_` variables of its call, and
//!   the variables the team's `driver.json` passes through.
//! - `kind`, `kubectl` and Docker run with the same minimal environment.
//! - A variable nobody names is not passed.
//!
//! ## Open
//!
//! - Which variables each family of tools needs beyond the base. `git` and
//!   `gh`, and `kind`, `kubectl` and Docker, each name their own, and neither
//!   list is settled.
//! - Which variables make up the locale. Until that is settled, they are
//!   `LANG`, `LC_ALL`, `LC_CTYPE` and `LC_MESSAGES`.
//!
//! Shared: used by `driver`, `local` and `delivery`.

pub mod allow;

use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Environment {
	pub variables: BTreeMap<String, String>,
}
