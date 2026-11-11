//! The image resolver that gives every component the same answer

mod resolve;

pub struct Answering {
	pub answer: &'static str,
}
