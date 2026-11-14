//! The candidates of a price list and an offer
//!
//! One function: it takes the price list and the team's offer, or none for
//! a project with no connection, and gives back the candidates.
//!
//! ## Rules
//!
//! - With no offer, every model of the three is offered at every effort.
//! - The models follow the three models nsx reads, and each model's efforts
//!   follow low, medium and high. An effort nsx does not know is left out.
//! - An offered model missing from the price list stops the command, as the
//!   spend table would.
//! - An offer that names none of the three models, or names them only at
//!   efforts nsx does not know, stops the command.

use crate::contract::{fence::EFFORTS, offer::Offer};
use crate::money::{
	candidate::Candidate, candidates::Candidates, error::Error, price::MODELS,
	price::Price,
};

impl Candidates {
	pub fn of(prices: &Price, offer: Option<&Offer>) -> Result<Self, Error> {
		let mut models = Vec::new();
		let mut named = false;

		for id in MODELS {
			// The providers that offer this model, or none to say that nothing
			// narrows it.
			let offering = offer.map(|offer| {
				offer
					.providers
					.iter()
					.filter(|offered| offered.models.iter().any(|model| model == id))
					.collect::<Vec<_>>()
			});
			let offered = offering
				.as_ref()
				.is_none_or(|offering| !offering.is_empty());

			if !offered {
				continue;
			}

			named = true;

			let listed = prices.models.iter().find(|model| model.id == id);
			let Some(model) = listed else {
				return Err(Error::Model {
					id: String::from(id),
				});
			};
			let efforts = EFFORTS
				.into_iter()
				.filter(|effort| {
					offering.as_ref().is_none_or(|offering| {
						offering.iter().any(|offered| {
							offered.efforts.iter().any(|known| known == effort)
						})
					})
				})
				.map(String::from)
				.collect::<Vec<_>>();

			if efforts.is_empty() {
				continue;
			}

			models.push(Candidate {
				model: model.clone(),
				efforts,
			});
		}

		if !named {
			return Err(Error::Unoffered);
		}

		if models.is_empty() {
			return Err(Error::Effortless);
		}

		Ok(Self {
			models,
		})
	}
}
