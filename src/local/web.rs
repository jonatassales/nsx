//! Downloads over the web
//!
//! The real `Download`: an HTTPS GET that follows the release hosts'
//! redirects.
//!
//! Definition only: the `Web` type, which carries nothing, and the bound on
//! one download. Its function is the file in `web/`.
//!
//! ## Rules
//!
//! - Only `https://` URLs are fetched.
//! - No header of nsx's goes with the request: no secret, no project name.
//! - A download waits at most five minutes.

pub mod get;

use std::time::Duration;

pub const PATIENCE: Duration = Duration::from_secs(300);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Web;
