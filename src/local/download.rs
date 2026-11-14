//! Downloading a file
//!
//! How nsx gets the bytes of a file it fetches from the web: the pinned
//! tools it keeps for the local cluster. A test gives its own bytes instead.
//!
//! Definition only: the `Download` trait, with one function: it takes a URL
//! and gives back the file's bytes.
//!
//! ## Rules
//!
//! - What comes back is never trusted as it is: whoever asked checks it
//!   against a pinned SHA-256 before using it.
//! - A URL that cannot be read is an error with one plain cause.

use std::future::Future;
use std::pin::Pin;

pub trait Download {
	fn get<'call>(
		&'call self,
		url: &'call str,
	) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, String>> + 'call>>;
}
