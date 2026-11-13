//! What reading the ticket and its links can refuse
//!
//! The errors of this folder, each one plain sentence that names the ticket,
//! the file or the link, so that the stop rule can print it as it is.
//!
//! Definition only: the `Error` type.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"nsx found no ticket {id}: no file named {id}.ticket.md is under .agents/ or .claude/."
	)]
	NoTicket { id: String },
	#[error(
		"nsx found more than one file named {id}.ticket.md under .agents/ and .claude/, and plans one ticket."
	)]
	Tickets { id: String },
	#[error("nsx could not search {}: {cause}.", path.display())]
	Unsearchable {
		path: PathBuf,
		cause: std::io::Error,
	},
	#[error(
		"nsx found no title in {}: a ticket opens with a heading.",
		path.display()
	)]
	NoTitle { path: PathBuf },
	#[error("nsx found no **Status:** line in {}.", path.display())]
	NoStatus { path: PathBuf },
	#[error(
		"nsx does not read the link {target}: a ticket links one .prd.md, one .dd.md, one .spec.md and one .gr.md."
	)]
	Kind { target: String },
	#[error("nsx found no {kind} among the ticket's links.")]
	Missing { kind: &'static str },
	#[error("nsx found more than one {kind} among the ticket's links.")]
	Twice { kind: &'static str },
	#[error(
		"nsx will not read the link {target}: a {kind} sits under {folder}/."
	)]
	Outside {
		target: String,
		kind: &'static str,
		folder: String,
	},
	#[error("nsx will not read the link {target}: it leaves the project.")]
	Climbs { target: String },
	#[error("nsx found no **{field}:** line in {}.", path.display())]
	Field { path: PathBuf, field: &'static str },
	#[error(
		"nsx could not read **{field}:** in {}: {value} is refused ({cause}).",
		path.display()
	)]
	Malformed {
		path: PathBuf,
		field: &'static str,
		value: String,
		cause: String,
	},
	#[error("nsx found no list under ## {section} in {}.", path.display())]
	Section {
		path: PathBuf,
		section: &'static str,
	},
	#[error(
		"nsx could not read the component {item} in {}: an item opens with a path in backticks, then an optional :port.",
		path.display()
	)]
	Component { path: PathBuf, item: String },
	#[error(
		"nsx cannot name a runtime after the ticket {id}: what follows its number must be lowercase letters, digits and dashes, 63 at most."
	)]
	Slug { id: String },
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
