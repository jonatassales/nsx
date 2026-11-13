//! The console's origin
//!
//! One function: it takes an address as a person wrote it and gives back the
//! console's origin, or nothing when nsx will not send a secret there.
//!
//! ## Rules
//!
//! - The address is `https`, or `http` on this machine alone (`localhost`,
//!   `127.0.0.1` or `[::1]`): the secret travels as a bearer.
//! - It names no user, no password, no query and no fragment, and no path
//!   beyond `/`.
//! - The origin is the scheme, the host and the port, as the URL standard
//!   writes them.

use reqwest::Url;

pub fn origin(address: &str) -> Option<String> {
	let url = Url::parse(address).ok()?;
	let host = url.host_str()?;
	let local = matches!(host, "localhost" | "127.0.0.1" | "[::1]");
	let safe = url.scheme() == "https" || (url.scheme() == "http" && local);
	let bare = url.username().is_empty()
		&& url.password().is_none()
		&& url.query().is_none()
		&& url.fragment().is_none()
		&& matches!(url.path(), "" | "/");

	if !safe || !bare {
		return None;
	}

	Some(url.origin().ascii_serialization())
}
