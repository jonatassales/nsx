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
//! - The base is `PATH`, `HOME`, `TMPDIR` and the locale (ADR 0002).
//! - The coding agent also gets the `LANGDRIFT_` variables of its call, and
//!   the variables the team's `driver.json` passes through (spec 0058).
//! - `kind`, `kubectl` and Docker run with the same minimal environment
//!   (spec 0062).
//! - A variable nobody names is not passed.
//!
//! ## Open
//!
//! - Which variables each family of tools needs beyond the base. ADR 0002
//!   says "what that tool family needs" and lists none for `git`, `gh`,
//!   `kind`, `kubectl` or Docker.
//! - Which variables make up the locale. Until a spec says, they are `LANG`,
//!   `LC_ALL`, `LC_CTYPE` and `LC_MESSAGES`.
//!
//! Shared: used by `driver`, `local` and `delivery`.

pub mod allow;

use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Environment {
	pub variables: BTreeMap<String, String>,
}
