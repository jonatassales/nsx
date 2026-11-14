//! One model the team offers, priced
//!
//! A model of the price list that the team offers on the console's
//! Settings → AI Models, with the efforts nsx knows that the team offers for
//! it.
//!
//! Definition only: the `Candidate` type: the model, with its prices, and
//! its efforts, in the order low, medium, high.

use crate::money::model::Model;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
	pub model: Model,
	pub efforts: Vec<String>,
}
