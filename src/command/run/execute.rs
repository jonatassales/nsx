//! The run, as its steps in order
//!
//! The script of `nsx agent run`. It reads top to bottom as phase 2 of the
//! simulation plays, and each step calls the folder that owns the line.
//!
//! One function: it takes the run command and plays the run to its last
//! line. It gives back how the run ended: on its last line, paused at the
//! gate, or stopped.
//!
//! The steps, in the simulation's order:
//!
//! - files and session: find the project root, set the window title
//! - money: read the price list
//! - flow: read the workflow, for each stage's skill
//! - handoff: choose the one intent that is authorized and has no completed
//!   run, or the one still proposed
//! - handoff: read the plan's card data
//! - console: open the reports, and in a connected project wait for the
//!   team's Play on a proposed intent (`authorize`)
//! - handoff: check the intent against the draft and the price list
//! - evidence: give the run its id
//! - money: open the run's money entry from the plan's spend table
//! - handoff: the opening comment and the intent's line (`scope`, `intent`)
//! - evidence: open `run.json`, with the outcome `running`
//! - `play`: the acts, from `estimate` on, watched for the person's
//!   interrupt
//! - `halt`: on a stop, take down what the run brought up, with no line
//! - evidence: close `run.json` with the outcome the run ended on, telling
//!   the console `done`, `paused` or `error`
//! - `remember`: leave the run's episode, once that outcome is `completed`
//!
//! Every write of `run.json` is told to the console in a connected project
//! (`evidence/record/write.rs`), with no line printed.
//!
//! ## Rules
//!
//! - Whatever stops the run after its opening, an interrupt included,
//!   `run.json` ends with the outcome `stopped`; a pause at the gate ends it
//!   `paused`.
//! - This file prints no line of its own: every line of phase 2 belongs to
//!   another folder, and the typed command is never echoed.
//! - A failure before the opening comment is a stop with nothing printed and
//!   nothing reserved.
//! - The over-budget pause is a shown ending: the run exits 0.

use crate::command::{applied::Applied, run::Run, session::Session};
use crate::console::report::Report;
use crate::contract::status::Status;
use crate::contract::{
	cards::Cards,
	decode::decode,
	outcome::Outcome,
	run::Run as Ran,
	step::{PENDING, Step},
};
use crate::evidence::{id::Id, record::Record};
use crate::files::{
	layout::Layout,
	root::Root,
	text::{DOCUMENT, text},
};
use crate::flow::workflow::{STAGES, Workflow};
use crate::handoff::{authorization::Authorization, error::Error};
use crate::money::{ledger::Ledger, price::Price};
use crate::session::{stop::Stop, title::Title};

impl Run {
	pub async fn execute(&self, session: &mut Session<'_>) -> Result<(), Stop> {
		let root = Root::find(&session.directory)?;
		let title = Title::of(&root);

		session.window.title(&title)?;

		let layout = Layout::under(&root);
		let prices = Price::load(&root, &layout)?;
		let workflow = Workflow::load(&root, &layout, &prices)?;
		let chosen = Authorization::choose(&root, &layout)?;
		let folder = layout.runtimes.join(&chosen.runtime_id);
		let planned = folder.join("plan.json");
		let written = text(&root, &planned, DOCUMENT)?;
		let cards: Cards = decode(&written).map_err(|cause| Error::Unread {
			path: planned,
			cause,
		})?;
		let mut report =
			Report::open(&root, &layout, &chosen.runtime_id, Status::Workflow)?;
		let intent = self
			.authorize(session, &root, &layout, chosen, &cards, &mut report)
			.await?;
		// The check, the run's id and its money entry: a stop among them is
		// told to the console, which may hold a team's Play for this run.
		let opening = || -> Result<_, Stop> {
			let authorization =
				Authorization::check(&root, &layout, intent.clone(), &prices)?;
			let applied = &authorization.intent;
			let id = Id::mint(&root, &layout, &applied.runtime_id)?;
			let ledger = Ledger::open(
				id.text(),
				cards.workflow.as_ref(),
				&applied.model,
				&applied.effort,
			)?;

			Ok((authorization, id, ledger))
		};
		let (authorization, id, mut ledger) = match opening() {
			Ok(opened) => opened,
			Err(stop) => {
				let told = (Some(&cards), Some(&intent), None);

				report.end(Status::Error, session.console, told).await?;

				return Err(stop);
			}
		};
		let applied = &authorization.intent;
		let stages = STAGES
			.iter()
			.map(|stage| Step {
				id: String::from(*stage),
				human: applied.human.ticked(stage),
				state: String::from(PENDING),
				..Step::default()
			})
			.collect();
		let mut record = Record {
			file: folder.join("run.json"),
			run: Ran {
				run: Some(id.text()),
				intent: Some(applied.clone()),
				stages: Some(stages),
				effort: None,
				safe_seat: None,
				runtime: None,
				delivery: None,
				outcome: Some(Outcome::Running),
			},
			cards: cards.clone(),
			report,
		};
		let applied = Applied {
			root,
			layout,
			authorization,
			cards,
			id,
			prices,
			workflow,
		};

		let opened = async {
			session.sink.print(&applied.authorization.scope())?;

			session.sink.print(&applied.authorization.line())?;
			record
				.write(&applied.root, session.console, |_run| {})
				.await?;

			Ok::<(), Stop>(())
		}
		.await;

		if let Err(stop) = opened {
			let told = (
				Some(&record.cards),
				record.run.intent.as_ref(),
				Some(&record.run),
			);

			record
				.report
				.end(Status::Error, session.console, told)
				.await?;

			return Err(stop);
		}

		// The run watches for the interrupt itself: it has a runtime to take
		// down and a record to close before it ends.
		let interrupt = session.interrupt;
		let played = tokio::select! {
			biased;

			() = interrupt.wait() => Err(Stop::interrupted()),
			played = self.play(session, &applied, &mut record, &mut ledger) => {
				played
			}
		};
		let played = match played {
			Ok(outcome) => Ok(outcome),
			Err(stop) => Err(self.halt(session, &applied, &mut record, stop).await),
		};
		let ended = match &played {
			Ok(outcome) => *outcome,
			Err(_stop) => Outcome::Stopped,
		};

		// The console reads the run's end from the status: done, the gate's
		// pause, or an error.
		record.report.status = match ended {
			Outcome::Completed => Status::Done,
			Outcome::Paused => Status::Paused,
			Outcome::Stopped | Outcome::Running => Status::Error,
		};
		// The end is told even when an earlier call failed.
		record.report.unavailable = false;

		let closed = record
			.write(&applied.root, session.console, |run| {
				run.outcome = Some(ended)
			})
			.await;

		// A run that stopped says why first: a record that could not be
		// closed is said after it, never in its place.
		match (played, closed) {
			(Ok(Outcome::Completed), Ok(())) => self.remember(&applied),
			(Ok(_outcome), Ok(())) => Ok(()),
			(Ok(_outcome), Err(unrecorded)) => Err(Stop::from(unrecorded)),
			(Err(stop), Ok(())) => Err(stop),
			(Err(stop), Err(unrecorded)) => Err(Stop {
				sentence: format!("{} {unrecorded}", stop.sentence),
			}),
		}
	}
}
