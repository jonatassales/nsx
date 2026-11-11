//! Waiting for the team's Play
//!
//! One function: it takes the reports' state, the console, the project's
//! root, the intent's file, the plan's cards and the pace of the wait. It
//! waits until the console's answer requests a play whose choice equals the
//! intent's, and gives back that Play, with the intent as it read then.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/microruntimes/runtimes/<runtime id>/intent.json`,
//!   again at each beat, so that a choice edited while the run waits reaches
//!   the console
//! - calls the console at each beat, reporting `held` with the records
//!   written so far
//!
//! ## Rules
//!
//! - The run waits only when the project has the connection, no intent is
//!   authorized, and exactly one recorded intent is proposed and has no
//!   completed run. A project with no connection is told so at once
//!   (`Error::Offline`), and the run stops as spec 0048 says.
//! - While it waits, it prints nothing.
//! - When the console stops answering, it keeps waiting and never starts on
//!   its own. There is no timeout.
//! - A Play pressed before the run is typed arrives in the answer to the
//!   run's first report, so the run starts with no further wait.
//! - A Play whose choice differs from the intent's is ignored, and the run
//!   keeps waiting.
//! - An intent that no longer reads is a stop, with its file.
//!
//! Spec 0081.

use std::path::Path;
use std::time::Duration;

use crate::console::{
	client::Client, error::Error, play::Play, report::Report,
};
use crate::contract::{
	asked::Asked, cards::Cards, choice::Choice, decode::decode, intent::Intent,
	status::Status,
};
use crate::files::{
	root::Root,
	text::{DOCUMENT, text},
};

impl Play {
	pub async fn wait(
		report: &mut Report,
		client: &dyn Client,
		root: &Root,
		file: &Path,
		cards: Option<&Cards>,
		beat: Duration,
	) -> Result<Self, Error> {
		report.status = Status::Held;

		loop {
			let written = text(root, file, DOCUMENT)?;
			let intent =
				decode::<Intent>(&written).map_err(|cause| Error::Intent {
					path: file.to_path_buf(),
					cause,
				})?;
			let choice = Choice::of(&intent);
			let answered = report.ask(client, cards, Some(&intent), None).await;
			let requested = match answered {
				Ok(requested) => requested,
				Err(Error::File(unwritten)) => return Err(Error::File(unwritten)),
				Err(Error::Offline) => return Err(Error::Offline),
				Err(_unavailable) => None,
			};
			let played = requested.is_some_and(|requested| {
				requested.state == Asked::Play
					&& requested.intent.as_ref() == Some(&choice)
			});

			if played {
				return Ok(Self {
					intent,
				});
			}

			tokio::time::sleep(beat).await;
		}
	}
}
