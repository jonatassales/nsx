//! Opening the judge's client
//!
//! One function: it takes the connection and gives back the client that
//! asks the console's judge.
//!
//! ## Rules
//!
//! - The HTTP client follows no redirect.

use reqwest::redirect::Policy;

use crate::console::connection::Connection;
use crate::judgment::{client::Client, error::Error};

impl Client {
	pub fn open(connection: Connection) -> Result<Self, Error> {
		let http = reqwest::Client::builder()
			.redirect(Policy::none())
			.build()
			.map_err(|cause| Error::Unavailable {
				cause: cause.without_url().to_string(),
			})?;

		Ok(Self {
			connection,
			http,
		})
	}
}
