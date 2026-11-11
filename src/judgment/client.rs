//! The console's judge, over the network
//!
//! The judge a connected project asks: the console's `/api/judge`, reached
//! with the connection `nsx init` recorded.
//!
//! Definition only: the `Client` type, which carries the connection and the
//! HTTP client. The model the judge answers with, the version of nsx's
//! words and the limits of a call sit beside the type. Its functions are
//! the files in `client/`.
//!
//! ## Rules
//!
//! - The bearer is the secret. It is never printed or logged.
//! - No redirect is followed.
//! - A question waits at most 130 seconds: the console gives its own model
//!   110.
//!
//! ## Open
//!
//! - Which model judges. Until the owner decides, it is `claude-sonnet-5-5`,
//!   and a console that does not offer it stops the plan with the console's
//!   own answer.
//!
//! Spec 0078.

pub mod ask;
pub mod open;

use std::time::Duration;

use crate::console::connection::Connection;

pub const MODEL: &str = "claude-sonnet-5-5";

pub const WORDS: &str = "nsx-1";

pub const ANSWER_TOKENS: u32 = 2000;

pub const PATIENCE: Duration = Duration::from_secs(130);

pub struct Client {
	pub connection: Connection,
	pub http: reqwest::Client,
}
