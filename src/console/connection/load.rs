//! Loading the connection
//!
//! One function: it takes the project's root and the paths under
//! `.langdrift/`, reads the project's private state and gives back the
//! connection, or none when the project has recorded no secret.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/agent/state/credentials/console.json`
//!
//! ## Rules
//!
//! - With no connection, nsx plans and runs offline: nothing is sent, and the
//!   plan's judge stays unavailable.
//! - The judge and the reports read the same connection.
//! - A file others on this machine can read is refused, and so is one whose
//!   origin nsx would not send a secret to.
//! - A file that does not read as a connection is refused without its
//!   reason: the decoder's words would quote the file, and so the secret.
//!
//! Spec 0081.

use crate::console::{connection::Connection, error::Error, origin::origin};
use crate::contract::decode::decode;
use crate::files::{
	inside::inside,
	layout::Layout,
	private::private,
	root::Root,
	text::{DOCUMENT, text},
};

impl Connection {
	pub fn load(root: &Root, layout: &Layout) -> Result<Option<Self>, Error> {
		let file = inside(root, &layout.credentials)?;

		if !file.exists() {
			return Ok(None);
		}

		if !private(&file)? {
			return Err(Error::Exposed {
				path: file,
			});
		}

		let written = text(root, &layout.credentials, DOCUMENT)?;
		let connection: Self =
			decode(&written).map_err(|_unsaid| Error::Unread {
				path: file,
			})?;

		if origin(&connection.origin).is_none() {
			return Err(Error::Origin);
		}

		Ok(Some(connection))
	}
}
