//! The project's files
//!
//! Everything nsx reads and writes on disk goes through here: finding the
//! project, naming the paths under `.langdrift/`, and reading and writing
//! without leaving a file half written or stepping outside the project.
//!
//! - `Root`: the project's root, the folder that holds `.langdrift/`
//! - `Layout`: the paths under `.langdrift/`
//! - `write`: writing a file whole or not at all
//! - `read`: reading a file up to a limit
//! - `text`: reading a document, bounded and UTF-8
//! - `within`: refusing a path that leaves the project
//! - `inside`: a path of the project, checked, given whole or from the root
//! - `hide`: keeping the private state out of Git
//! - `Error`: what can go wrong with the project's files
//!
//! ## Rules
//!
//! - `.langdrift/` holds LangDrift's workflow, skills, harness, prices and
//!   records. The client's tickets, documents and code stay outside it.
//! - A failure here is a stop: nothing more is printed, one sentence without
//!   a secret goes to stderr, and the command exits 1.
//!
//! Shared: used by every folder that reads or writes a file of the project.

pub mod error;
pub mod folder;
pub mod hide;
pub mod inside;
pub mod layout;
pub mod private;
pub mod read;
pub mod root;
pub mod sealed;
pub mod text;
pub mod within;
pub mod write;
