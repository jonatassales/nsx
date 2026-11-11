//! Opening the console's client
//!
//! One function: it takes the connection and gives back the client that
//! reaches the console over the network.
//!
//! ## Rules
//!
//! - The HTTP client follows no redirect.
//!
//! Spec 0081.

use reqwest::redirect::Policy;

use crate::console::{connection::Connection, error::Error, remote::Remote};

impl Remote {
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
