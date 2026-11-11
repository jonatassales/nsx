//! One line as a golden freezes it: its id, the word of its kind and its text

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frozen {
	pub id: String,
	pub kind: String,
	pub text: String,
}
