//! A lowercase DNS label
//!
//! What a runtime's id must be, so that it is also a folder's name and a
//! Kubernetes label's value: the fence's `runtimeId`, and the ticket's slug
//! it comes from.
//!
//! One function: it takes a text and says whether it is a lowercase DNS
//! label: lowercase letters, digits and inner dashes, 63 characters at most.
//!
//! Spec 0077.

const MAXIMUM: usize = 63;

pub fn label(text: &str) -> bool {
	let edge = |character: char| {
		character.is_ascii_lowercase() || character.is_ascii_digit()
	};
	let inner = text
		.chars()
		.all(|character| edge(character) || character == '-');

	text.len() <= MAXIMUM
		&& text.starts_with(edge)
		&& text.ends_with(edge)
		&& inner
}
