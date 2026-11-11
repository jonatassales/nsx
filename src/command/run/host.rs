//! Bringing the runtime up
//!
//! One function: it takes what the run applies, the run's record and its
//! execution ledger, brings the components of the applied revision up and
//! prints that the runtime is ready. It gives back the runtime's state.
//!
//! The steps:
//!
//! - contract: read the draft's fence again, for the components
//! - local: one Deployment per component, brought up in the run's space
//! - local: the ready line (`runtimeReady`), then the record's runtime
//!   section
//!
//! ## Rules
//!
//! - The draft is read here, where it is used. A draft that moved to another
//!   revision since the intent was authorized is a stop: the run brings up
//!   the revision it applies and no other.
//! - No line prints until every Deployment is available.
//!
//! Spec 0077.

use crate::command::{applied::Applied, run::Run, session::Session};
use crate::contract::fence::Fence;
use crate::evidence::record::Record;
use crate::files::text::{DOCUMENT, text};
use crate::handoff::error::Error as Unread;
use crate::local::{
	deployment::Deployment, error::Error, execution::Execution, state::State,
};
use crate::session::stop::Stop;

impl Run {
	pub async fn host(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
		record: &mut Record,
		execution: &mut Execution,
	) -> Result<State, Stop> {
		let root = &applied.root;
		let layout = &applied.layout;
		let intent = &applied.authorization.intent;
		let draft = layout.runtimes.join(&intent.runtime_id).join("runtime.md");
		let written = text(root, &draft, DOCUMENT)?;
		let fence = Fence::parse(&written).map_err(|cause| Unread::Draft {
			cause,
		})?;

		if fence.revision != intent.revision {
			return Err(Stop::from(Error::Revised {
				applied: intent.revision,
				draft: fence.revision,
			}));
		}

		let deployments = fence
			.components
			.iter()
			.map(Deployment::of)
			.collect::<Result<Vec<_>, Error>>()?;

		execution
			.up(root, layout, session.cluster, &deployments)
			.await?;

		let state = State::of(intent, &deployments);

		session.sink.print(&state.line())?;
		record
			.write(root, session.console, |run| state.card(run))
			.await?;

		Ok(state)
	}
}
