//! Installing the skills
//!
//! One function: it lays down the six skills in the project, one folder per
//! skill.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/skills/grilling/`, `.langdrift/skills/to-spec/`,
//!   `.langdrift/skills/to-tickets/`, `.langdrift/skills/implement/`,
//!   `.langdrift/skills/tdd/` and `.langdrift/skills/code-review/`
//!
//! ## Rules
//!
//! - `nsx init` calls it, and it prints nothing.
//! - A file already there is kept byte for byte, so installing twice changes
//!   nothing.
//! - The skills are taken from the ones this repository runs, in
//!   `.claude/skills/`.
