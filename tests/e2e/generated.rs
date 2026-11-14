//! A ticket of its own for each end-to-end run
//!
//! Each run plans a ticket no run planned before, so that the console's
//! runtimes are told apart: a four-digit number and a slug that starts with
//! `e2e-`, made of two generated words.

pub mod id;
pub mod make;

pub struct Generated {
	pub number: String,
	pub slug: String,
}
