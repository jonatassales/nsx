//! Getting a file over HTTPS
//!
//! One function: it takes a URL and gives back the bytes the host answers,
//! after its redirects.
//!
//! ## Reads and writes
//!
//! - requests the URL, with no header of nsx's
//!
//! ## Rules
//!
//! - A URL that is not `https://`, a host that cannot be reached, or a
//!   status that is not a success is an error naming the cause, never the
//!   bytes of an error page.

use std::future::Future;
use std::pin::Pin;

use crate::local::{
	download::Download,
	web::{PATIENCE, Web},
};

impl Download for Web {
	fn get<'call>(
		&'call self,
		url: &'call str,
	) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, String>> + 'call>> {
		Box::pin(async move {
			if !url.starts_with("https://") {
				return Err(String::from("its address is not https"));
			}

			let failed = |cause: reqwest::Error| cause.without_url().to_string();
			let client = reqwest::Client::builder()
				.timeout(PATIENCE)
				.build()
				.map_err(failed)?;

			let answered = client.get(url).send().await.map_err(failed)?;

			let fetched = answered.error_for_status().map_err(failed)?;

			let bytes = fetched.bytes().await.map_err(failed)?;

			Ok(bytes.to_vec())
		})
	}
}
