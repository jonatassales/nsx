//! Making sure Docker answers
//!
//! One function: before the cluster is first reached, it asks Docker
//! whether it answers, starts it when it does not, and waits until it does.
//!
//! ## Reads and writes
//!
//! - runs `docker info`, through `answers.rs`
//! - starts Docker, through `rouse.rs`
//!
//! ## Rules
//!
//! - It runs once for each `Kind`, and prints nothing: the simulation shows
//!   no line about Docker.
//! - After starting Docker it asks again after each pause, at most `WAKING`
//!   times: three minutes at the real pause.
//! - A Docker that still does not answer is a stop under the stop rule.

use crate::local::{
	error::Error,
	kind::{Kind, WAKING},
};

impl Kind<'_> {
	pub async fn wake(&self) -> Result<(), Error> {
		if self.awake.get() {
			return Ok(());
		}

		if self.answers().await? {
			self.awake.set(true);

			return Ok(());
		}

		self.rouse().await?;

		for _asking in 0..WAKING {
			tokio::time::sleep(self.pause).await;

			if self.answers().await? {
				self.awake.set(true);

				return Ok(());
			}
		}

		Err(Error::Asleep)
	}
}
