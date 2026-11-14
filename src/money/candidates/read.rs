//! Reading the candidates from the console
//!
//! One function: it takes the console and the price list, reads the team's
//! offer and gives back the candidates.
//!
//! ## Reads and writes
//!
//! - calls the console's tool `langdrift.account.offer` once
//!
//! ## Rules
//!
//! - A project with no connection reads no offer: every model and effort is
//!   a candidate, and the plan stops at its first judgment as before.
//! - A console that cannot answer the offer stops the plan before its first
//!   line, with the console's reason and the place the team sets the offer.

use crate::console::{client::Client, error::Error as Console};
use crate::money::{candidates::Candidates, error::Error, price::Price};

impl Candidates {
	pub async fn read(
		console: &dyn Client,
		prices: &Price,
	) -> Result<Self, Error> {
		let offer = match console.offer().await {
			Ok(offer) => Some(offer),
			Err(Console::Offline) => None,
			Err(cause) => {
				return Err(Error::Offer {
					cause: cause.to_string(),
				});
			}
		};

		Self::of(prices, offer.as_ref())
	}
}
