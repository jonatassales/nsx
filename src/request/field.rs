//! A field line of a document
//!
//! How a team's document states a fact nsx reads: the tracker's field-line
//! convention, `**Field:** value`, at the start of a line.
//!
//! One function: it takes a document's text and a field's name, and gives
//! back the value of that field's line, or nothing when the document has no
//! such line or leaves it empty.
//!
//! ## Rules
//!
//! - The first line of the field counts. No model reads a document.

pub fn field<'text>(text: &'text str, name: &str) -> Option<&'text str> {
	let opening = format!("**{name}:**");

	text
		.lines()
		.find_map(|line| line.strip_prefix(opening.as_str()))
		.map(str::trim)
		.filter(|value| !value.is_empty())
}
