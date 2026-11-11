//! Starting to watch for an interrupt
//!
//! One function: it starts watching the interrupt, the request to end and
//! the terminal's closing, and gives back the signal to wait on.
//!
//! ## Rules
//!
//! - From here on none of the three ends nsx by itself: the command that
//!   waits decides how it stops.
//! - A signal that cannot be watched is a failure of its own, said before
//!   the command starts.
//!
//! Spec 0077.

use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::Mutex;

use crate::session::signal::Signal;

impl Signal {
	pub fn watch() -> std::io::Result<Self> {
		let streams = [
			signal(SignalKind::interrupt())?,
			signal(SignalKind::terminate())?,
			signal(SignalKind::hangup())?,
		];

		Ok(Self {
			streams: Mutex::new(streams),
		})
	}
}
