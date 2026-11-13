//! The stages of the run, each with its checkpoint
//!
//! The workflow's stages play in order, from Planning to Review. A ticked
//! stage first waits for the owner. A stage with a skill makes one call to
//! the team's coding agent. The dev ticket is the person's own work.
//!
//! One function: it takes what the run applies, the run's record and what
//! the acts share, and plays the stages, each step calling the folder that
//! owns the line. It leaves the usage record filled, or gives back the stop.
//!
//! The steps, in the simulation's order:
//!
//! - Planning: flow waits for the owner (`waiting-planning`,
//!   `confirmed-planning`), then driver and skills (`grilling`)
//! - Spec: when ticked, flow waits (`waiting-spec`, `confirmed-spec`), then
//!   driver and skills (`toSpec`)
//! - Tickets: when ticked, flow waits (`waiting-tickets`,
//!   `confirmed-tickets`), then driver and skills (`toTickets`)
//! - Implement: when ticked, flow waits (`waiting-implement`,
//!   `confirmed-implement`), then hook (`hook`), driver and skills
//!   (`implement`)
//! - Dev ticket: seat waits for the person (`devTicketWaiting`,
//!   `devTicketClosed`)
//! - Review: driver and skills (`codeReview`), with no waiting line
//!
//! ## Rules
//!
//! - What the stages need is checked before any of them waits (`prepare`).
//! - A stage is `waiting` while its checkpoint waits, `active` while it
//!   works and `done` once its line has printed.
//! - Planning always has a person. Review's tick only decides how the run
//!   ends.
//! - No stage call is allowed while the dev ticket waits, and no runtime is
//!   up while anyone waits.
//! - Before a stage's work, after its checkpoint, the team's Pause holds the
//!   run without a line (`console/hold`).

use crate::command::{
	applied::Applied, run::Run, session::Session, work::Work,
};
use crate::console::hold::Hold;
use crate::contract::step::{ACTIVE, WAITING};
use crate::evidence::record::Record;
use crate::flow::checkpoint::Checkpoint;
use crate::hook::effort::Effort;
use crate::session::stop::Stop;

const CHECKED: [&str; 4] = ["planning", "spec", "tickets", "implement"];

impl Run {
	pub async fn stages(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
		record: &mut Record,
		work: &mut Work,
	) -> Result<(), Stop> {
		let root = &applied.root;
		let intent = &applied.authorization.intent;

		for entry in &applied.workflow.stages {
			let stage = entry.id.as_str();
			let waits = CHECKED.contains(&stage) && intent.human.ticked(stage);

			// Merge is never played.
			if stage == "merge" {
				continue;
			}

			if waits {
				let waiting = Checkpoint::Waiting {
					stage: String::from(stage),
				};

				session.sink.print(&waiting.line())?;
				record
					.stage(root, session.console, stage, |step| {
						step.state = String::from(WAITING)
					})
					.await?;

				let confirmed = Checkpoint::wait(stage, session.answers)?;

				session.sink.print(&confirmed.line())?;
				record
					.stage(root, session.console, stage, |step| {
						step.confirmed_by = Some(String::from("owner"));
					})
					.await?;
			}

			// The team's Pause holds the run here, before the stage's work.
			let records = (
				Some(&record.cards),
				record.run.intent.as_ref(),
				Some(&record.run),
			);

			Hold
				.wait(&mut record.report, session.console, records, session.beat)
				.await?;

			// The dev ticket is the person's own.
			if stage == "dev-ticket" {
				self.person(session, applied, record, work).await?;

				continue;
			}

			record
				.stage(root, session.console, stage, |step| {
					step.state = String::from(ACTIVE)
				})
				.await?;

			if stage == "implement" {
				let suggested = applied
					.cards
					.analysis
					.as_ref()
					.and_then(|analysis| analysis.suggestion.as_ref())
					.map(|suggestion| {
						(suggestion.model.clone(), suggestion.effort.clone())
					})
					.unwrap_or_default();
				// The line says the effort is recorded: it prints once it is.
				let effort = Effort::record(
					root,
					session.console,
					intent,
					suggested,
					session.clock,
					record,
				)
				.await?;

				session.sink.print(&effort.line())?;

				work.effort = Some(effort);
			}

			self.call(session, applied, record, work, entry).await?;
		}

		Ok(())
	}
}
