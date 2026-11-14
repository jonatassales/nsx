//! The console, over the network
//!
//! The team's console as the binary reaches it: one HTTP endpoint,
//! `<origin>/api/mcp`, that answers one JSON-RPC message per request.
//!
//! Definition only: the `Remote` type, which carries the connection and the
//! HTTP client. The protocol version and the time a call may take sit beside
//! the type. Its functions are the files in `remote/`.
//!
//! ## Rules
//!
//! - Two tools answer here: the report and the team's offer.
//! - The bearer is the secret. It is never printed or logged.
//! - No redirect is followed: the secret goes to the recorded origin alone.
//! - A call waits at most 4 seconds.

pub mod call;
pub mod client;
pub mod open;

use std::time::Duration;

use crate::console::connection::Connection;

pub const PROTOCOL: &str = "2025-11-25";

pub const PATIENCE: Duration = Duration::from_secs(4);

pub struct Remote {
	pub connection: Connection,
	pub http: reqwest::Client,
}
