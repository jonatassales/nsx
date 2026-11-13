//! What `runtime.md` does not carry
//!
//! The plan states that gpu and disk are not `runtime.md` fields and that none
//! was requested, so that nobody expects a limit the file cannot hold. The
//! statement is true because the schema has no gpu or disk field and no
//! extracted fact requests one.
//!
//! Definition only: the `Limits` type: the limits statement of the sizing
//! report, gpu and disk as not fields, none requested. It carries nothing:
//! the statement is the same for every plan. Its functions are the files in
//! `limits/`.
//!
//! ## Rules
//!
//! - The statement is the schema's fact. No fact is scanned for "gpu" or
//!   "disk".

pub mod card;
pub mod line;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Limits;
