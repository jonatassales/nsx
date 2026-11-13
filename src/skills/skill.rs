//! The skill a stage follows
//!
//! One of the skills LangDrift ships with its workflow: `grilling` for
//! Planning, `to-spec` for Spec, `to-tickets` for Tickets, `implement` and
//! `tdd` for Implement, `code-review` for Review. They are the skills this
//! repository runs on its own work. Each one ends by writing its stage's
//! report.
//!
//! Definition only: the `Skill` type, which names one of the six skills and
//! the folder `nsx init` lays down for it. Its functions are the files in
//! `skill/`.
//!
//! ## Rules
//!
//! - The skills work on the client's own tickets and documents, which stay in
//!   the client's `.agents/` or `.claude/` folder.
//! - The stage driver runs them with the chosen model and effort.
//! - What a skill does inside the client's project, beyond the report it
//!   writes, is out of scope.

pub mod install;
