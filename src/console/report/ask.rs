//! Asking the console
//!
//! One function: it takes the console and the records written so far, sends
//! one report, keeps the runtime's id the console answers and gives back the
//! team's pending request, if any.
//!
//! ## Reads and writes
//!
//! - calls the console's report tool, through the client
//! - writes `.langdrift/agent/state/console/<runtime id>.id`, the first time
//!   the console answers the runtime's id
//!
//! ## Rules
//!
//! - It prints nothing, whatever the console answers.
//! - It always calls, even once the console was found unavailable: the run
//!   asks at its hold points, while it waits for the team and when it ends,
//!   and each ask is its own chance (spec 0076). A console that answers is
//!   available again for the records' sends.
//! - A console that no longer knows the kept id is asked once more with
//!   none: it opens the runtime anew, and the new id is kept.
//! - A call that fails gives back its failure, and the caller decides what
//!   it means. Only keeping the id can fail as a file.
//!
//! Spec 0081.

use crate::console::{client::Client, error::Error, report::Report};
use crate::contract::{
	cards::Cards, intent::Intent, report::Report as Sent, requested::Requested,
	run::Run,
};
use crate::files::sealed::sealed;

impl Report {
	pub async fn ask(
		&mut self,
		client: &dyn Client,
		cards: Option<&Cards>,
		intent: Option<&Intent>,
		run: Option<&Run>,
	) -> Result<Option<Requested>, Error> {
		let report = |runtime: Option<String>| Sent {
			runtime_id: runtime,
			status: self.status,
			plan: cards.cloned(),
			intent: intent.cloned(),
			run: run.cloned(),
		};

		let answered = client.report(&report(self.runtime.clone())).await;

		// A console that no longer knows the kept id is asked again with
		// none, and names the runtime anew.
		let forgotten =
			self.runtime.is_some() && matches!(&answered, Err(Error::Unknown));
		let reply = if forgotten {
			client.report(&report(None)).await?
		} else {
			answered?
		};
		let renamed = self.runtime.as_ref() != Some(&reply.runtime_id);

		self.runtime = Some(reply.runtime_id.clone());
		self.unavailable = false;

		if renamed {
			sealed(&self.kept, reply.runtime_id.as_bytes())?;
		}

		Ok(reply.requested)
	}
}
