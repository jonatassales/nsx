//! Waiting for the team's authorization
//!
//! One function: it takes the chosen intent, the plan's cards and where the
//! reports to the console stand. A proposed intent in a connected project
//! waits, silently, for the team's Play, and is then authorized. It gives
//! back the intent the run goes on with.
//!
//! The steps:
//!
//! - console: wait for a Play whose choice equals the intent's, reporting
//!   `held`
//! - handoff: write `status: authorized` into `intent.json`, as it read when
//!   the Play matched
//!
//! ## Rules
//!
//! - An intent authorized by hand goes on at once.
//! - A project with no connection does not wait: its proposed intent goes
//!   on as it is, and the authorization's check stops the run with today's
//!   sentence.
//! - The person's interrupt while the run waits, or an intent that no longer
//!   reads, is a stop. The console is told `error` once, and nothing waits
//!   for its answer beyond the client's own bound.
//! - It prints nothing.
//!
//! Spec 0077.

use crate::command::{run::Run, session::Session};
use crate::console::{error::Error, play::Play, report::Report};
use crate::contract::{cards::Cards, intent::Intent, status::Status};
use crate::files::{layout::Layout, root::Root};
use crate::handoff::authorization::Authorization;
use crate::session::stop::Stop;

const PROPOSED: &str = "proposed";

impl Run {
	pub async fn authorize(
		&self,
		session: &Session<'_>,
		root: &Root,
		layout: &Layout,
		intent: Intent,
		cards: &Cards,
		report: &mut Report,
	) -> Result<Intent, Stop> {
		if intent.status != PROPOSED {
			return Ok(intent);
		}

		let file = layout.runtimes.join(&intent.runtime_id).join("intent.json");
		let interrupt = session.interrupt;
		let waited = tokio::select! {
			biased;

			() = interrupt.wait() => None,
			waited = Play::wait(
				report,
				session.console,
				root,
				&file,
				Some(cards),
				session.beat,
			) => Some(waited),
		};

		let told = (Some(cards), Some(&intent), None);

		let Some(waited) = waited else {
			report.end(Status::Error, session.console, told).await?;

			return Err(Stop::interrupted());
		};

		match waited {
			Ok(play) => {
				let authorized = Authorization::grant(root, &file, play.intent)?;

				report.status = Status::Workflow;

				Ok(authorized)
			}
			Err(Error::Offline) => Ok(intent),
			Err(failure) => {
				report.end(Status::Error, session.console, told).await?;

				Err(Stop::from(failure))
			}
		}
	}
}
