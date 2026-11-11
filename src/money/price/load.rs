//! Reading the price list
//!
//! One function: it takes the project's root and the paths under
//! `.langdrift/`, reads the project's price list and gives it back.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/finops/prices.json`
//!
//! ## Rules
//!
//! - A missing or unreadable price list stops the command before the line that
//!   would need it (the stop rule).
//! - `nsx init` writes the file with the three models. This function only
//!   reads it.
//! - A field the price list does not define is refused, and so is a price
//!   list that is a symbolic link.
//! - A negative price is refused: no amount made from it would be a spend.
//! - A model beyond the three nsx reads is refused, and so is a model listed
//!   twice. The spend table prices each of the three once, so a fourth could
//!   be suggested with no row, and a second price would be ignored.
//!
//! Spec 0079.

use crate::contract::decode::decode;
use crate::files::{
	layout::Layout,
	root::Root,
	text::{DOCUMENT, text},
};
use crate::money::{
	cents::Cents,
	error::Error,
	price::{MODELS, Price},
};

impl Price {
	pub fn load(root: &Root, layout: &Layout) -> Result<Self, Error> {
		let written = text(root, &layout.prices, DOCUMENT)?;

		let prices: Self = decode(&written).map_err(|cause| Error::Prices {
			path: layout.prices.clone(),
			cause,
		})?;
		let negative = prices.models.iter().find(|model| {
			model.input_cents < Cents(0) || model.output_cents < Cents(0)
		});

		if let Some(model) = negative {
			return Err(Error::Negative {
				path: layout.prices.clone(),
				model: model.id.clone(),
			});
		}

		let beyond = prices
			.models
			.iter()
			.find(|model| !MODELS.contains(&model.id.as_str()));

		if let Some(model) = beyond {
			return Err(Error::Beyond {
				path: layout.prices.clone(),
				model: model.id.clone(),
			});
		}

		let twice = prices.models.iter().enumerate().find(|(at, model)| {
			prices.models[..*at]
				.iter()
				.any(|earlier| earlier.id == model.id)
		});

		if let Some((_at, model)) = twice {
			return Err(Error::Twice {
				path: layout.prices.clone(),
				model: model.id.clone(),
			});
		}

		Ok(prices)
	}
}
