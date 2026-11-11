//! An image reference
//!
//! One function: it says whether a text is an image reference: plain text
//! with no space, tab or line break in it.
//!
//! ## Rules
//!
//! - It is the one rule for an image, wherever one is checked: when the plan
//!   resolves it and when the fence is validated.
//! - It reads the shape only. Whether a registry holds the image is nobody's
//!   to say during a plan.
//!
//! Spec 0077.

use crate::contract::plain::plain;

pub fn reference(text: &str) -> bool {
	plain(text) && !text.contains([' ', '\t', '\r', '\n'])
}
