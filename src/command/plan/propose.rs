//! The end of the plan act: the draft and its proposal
//!
//! The plan closes by validating the draft, writing it, and proposing it to
//! the team with a recorded intent. It proposes and runs nothing: the team
//! decides before anything is built or paid for.
//!
//! One function: it takes what the plan read before its first line, the
//! resolved components, their sizing, the suggestion and the card data's
//! writer, and plays the closing steps. It gives back how they ended:
//! proposed, or stopped.
//!
//! The steps, in the simulation's order:
//!
//! - runtime: validate the draft against the schema, then write `runtime.md`
//!   (`validate`)
//! - handoff: record the proposed intent, set the Plan card's authorization
//!   and evidence, and print the plan's last result (`draft`)
//!
//! ## Rules
//!
//! - The intent is recorded only after the draft is written.
//! - The console is told the proposed intent last, with the cards.
//! - A plan that stops writes no draft and no intent.
//! - nsx records the team's authorization, it never decides it.

use crate::command::{
	brief::Brief, desk::Desk, plan::Plan, resolved::Resolved, session::Session,
};
use crate::handoff::proposal::Proposal;
use crate::judgment::suggestion::Suggestion;
use crate::runtime::draft::Draft;
use crate::session::stop::Stop;

impl Plan {
	pub async fn propose(
		&self,
		session: &mut Session<'_>,
		brief: &Brief,
		resolved: &Resolved,
		suggestion: &Suggestion,
		desk: &mut Desk,
	) -> Result<(), Stop> {
		let root = &brief.root;
		let runtime = &brief.ticket.slug;
		let folder = brief.layout.runtimes.join(runtime);
		let file = folder.join("runtime.md");
		let revision = resolved.revision;
		let draft = Draft::of(
			runtime,
			revision,
			suggestion,
			&resolved.inventory.components,
			&resolved.sizing,
		);
		let errors = draft.validate()?;

		session.sink.print(&Draft::line(errors))?;
		desk
			.record(root, session.console, |cards| Draft::card(errors, cards))
			.await?;

		draft.write(root, &file, errors)?;

		let proposal = Proposal {
			ticket: brief.ticket.id.clone(),
			runtime: runtime.clone(),
			revision: revision.0,
			choice: brief.workflow.defaults.clone(),
		};

		proposal.write(root, &folder.join("intent.json"))?;

		session.sink.print(&proposal.line())?;

		desk
			.record(root, session.console, |cards| proposal.card(cards))
			.await?;

		let intent = proposal.intent();

		desk
			.report
			.send(
				session.console,
				Some(&desk.writer.cards),
				Some(&intent),
				None,
			)
			.await?;

		Ok(())
	}
}
