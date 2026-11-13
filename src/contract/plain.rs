//! Plain text
//!
//! What a text nsx prints or names a thing by must be: words on one line.
//!
//! One function: it takes a text and says whether it is plain: not blank, and
//! free of control characters, a line break among them.

pub fn plain(text: &str) -> bool {
	!text.trim().is_empty() && !text.chars().any(char::is_control)
}
