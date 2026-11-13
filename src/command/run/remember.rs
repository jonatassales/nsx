//! Leaving the run's episode
//!
//! One function: it takes what a completed run applied and writes the
//! episode the next plans can recall: the ticket with its intent, the
//! components the plan resolved and the resource it suggested.
//!
//! The steps:
//!
//! - request: find the ticket again and take its intent, the text a plan
//!   embeds
//! - rag: write the episode and add it to the index
//!
//! ## Rules
//!
//! - It runs once, after a run whose outcome is `completed`. A run that
//!   paused or stopped leaves no episode.
//! - Nothing was measured: the episode says so with `null`, never with a
//!   figure.
//! - What the episode keeps is what the plan's cards hold. A card without a
//!   component's stack or role, or without the suggested resource, stops
//!   here: nothing is written in its place.
//! - It prints nothing.

use crate::command::{applied::Applied, run::Run};
use crate::files::text::{DOCUMENT, text};
use crate::rag::{
	embedder::Embedder, episode::Episode, error::Error, part::Part,
	resource::Resource,
};
use crate::request::ticket::Ticket;
use crate::session::stop::Stop;

impl Run {
	pub fn remember(&self, applied: &Applied) -> Result<(), Stop> {
		let root = &applied.root;
		let intent = &applied.authorization.intent;
		let found = Ticket::resolve(root, &intent.ticket)?;
		let written = text(root, &found, DOCUMENT)?;
		let ticket = Ticket::parse(&intent.ticket, &found, &written)?;
		let unplanned = |what: &'static str| Error::Unplanned {
			what,
		};
		let planned = applied.cards.plan.as_ref();
		let rows = planned
			.and_then(|plan| plan.components.as_ref())
			.and_then(|components| components.rows.as_ref())
			.ok_or_else(|| unplanned("component"))?;
		let suggested = planned
			.and_then(|plan| plan.sizing.as_ref())
			.and_then(|sizing| sizing.resource.as_ref())
			.ok_or_else(|| unplanned("suggested resource"))?;
		let components = rows
			.iter()
			.map(|row| {
				let stack = row.stack.clone().ok_or_else(|| unplanned("stack"))?;
				let role = row.role.clone().ok_or_else(|| unplanned("role"))?;

				Ok(Part {
					path: row.path.clone(),
					stack,
					role,
				})
			})
			.collect::<Result<Vec<_>, Error>>()?;
		let replicas = u64::try_from(suggested.replicas).map_err(|_negative| {
			Error::Replicas {
				count: suggested.replicas,
			}
		})?;
		let episode = Episode {
			id: String::new(),
			slug: intent.runtime_id.clone(),
			ticket: intent.ticket.clone(),
			intent: ticket.intent(&written),
			resource: Resource {
				kind: suggested.kind.clone(),
				count: suggested.count,
				replicas,
			},
			components,
			measured: None,
		};

		episode.write(root, &applied.layout, &Embedder)?;

		Ok(())
	}
}
