//! The plan, as its steps in order
//!
//! The script of `nsx agent plan <ticket>`. It reads top to bottom as phase 1
//! of the simulation plays, and each step calls the folder that owns the line.
//!
//! One function: it takes the plan command, with the ticket's bare id, and
//! plays the plan to its last line. It gives back how the plan ended: on its
//! last line, or stopped.
//!
//! The steps, in the simulation's order:
//!
//! - files and session: find the project root, set the window title
//! - request: resolve the ticket, read its header and its links, and read
//!   the design doc for the budget per run
//! - money: read the price list and the forecast, and build the spend table
//! - flow: read the workflow, for the default choice the plan proposes
//! - handoff: write the ticket's header, the `workflow` section (`flow`,
//!   `money`) and the EPISODES threshold (`rag`) to `plan.json`, before any
//!   line prints
//! - here: the `scope` comment
//! - request: read the ticket and its links (`readTicket`, `links`)
//! - `analyze`: the analysis act, from `analysis` to `suggest`
//! - `resolve`: the plan act, from `planning` to `price`
//! - `propose`: the end of the plan act, `validate` and `draft`
//! - here: the `handoff` comment
//!
//! ## Simulation
//!
//! - `scope`: `# plan only · nothing is built, started or paid for in this step`
//! - `handoff`: `# next: once the team authorizes, nsx agent run applies the recorded intent`
//!
//! ## Rules
//!
//! - `scope` is the first line nsx prints: the typed command is never echoed.
//! - Lines print when their work is done. No line is delayed to imitate the
//!   simulation's pacing.
//! - A plan that stops writes no draft and no intent.
//!
//! Spec 0077.

use crate::command::{brief::Brief, desk::Desk, plan::Plan, session::Session};
use crate::console::report::Report;
use crate::contract::cards::Cards;
use crate::contract::status::Status;
use crate::files::{
	layout::Layout,
	root::Root,
	text::{DOCUMENT, text},
};
use crate::flow::workflow::Workflow;
use crate::handoff::writer::Writer;
use crate::money::{
	budget::Budget, forecast::Forecast, price::Price, spend::Spend,
};
use crate::rag::threshold::THRESHOLD;
use crate::request::{design::Design, links::Links, ticket::Ticket};
use crate::session::{line::Line, stop::Stop, title::Title};

impl Plan {
	pub async fn execute(&self, session: &mut Session<'_>) -> Result<(), Stop> {
		let root = Root::find(&session.directory)?;
		let title = Title::of(&root);

		session.window.title(&title)?;

		let found = Ticket::resolve(&root, &self.ticket)?;
		let written = text(&root, &found, DOCUMENT)?;
		let ticket = Ticket::parse(&self.ticket, &found, &written)?;
		let links = Links::parse(&root, &ticket, &written)?;
		let design = Design::read(&root, &links.design)?;
		let layout = Layout::under(&root);
		let prices = Price::load(&root, &layout)?;
		let workflow = Workflow::load(&root, &layout, &prices)?;
		let forecast = Forecast::load(&root, &layout)?;
		let spend = Spend::build(&prices, &forecast, &Budget::of(&design))?;
		let report = Report::open(&root, &layout, &ticket.slug, Status::Analysis)?;
		let mut desk = Desk {
			writer: Writer {
				file: layout.runtimes.join(&ticket.slug).join("plan.json"),
				cards: Cards::default(),
			},
			report,
		};
		let brief = Brief {
			root,
			layout,
			ticket,
			written,
			links,
			design,
			prices,
			workflow,
		};
		let root = &brief.root;

		desk
			.record(root, session.console, |cards| {
				brief.ticket.card(cards);
				THRESHOLD.card(cards);
				spend.card(cards);
			})
			.await?;

		session.sink.print(&Line::comment(
			"scope",
			"plan only · nothing is built, started or paid for in this step",
		))?;

		session.sink.print(&brief.ticket.line())?;

		session
			.sink
			.print(&brief.links.line(&brief.ticket.status))?;
		desk
			.record(root, session.console, |cards| brief.links.card(cards))
			.await?;

		let (spec, suggestion) = self.analyze(session, &brief, &mut desk).await?;
		let resolved = self.resolve(session, &brief, &spec, &mut desk).await?;

		self
			.propose(session, &brief, &resolved, &suggestion, &mut desk)
			.await?;

		session.sink.print(&Line::comment(
			"handoff",
			"next: once the team authorizes, nsx agent run applies the recorded intent",
		))
	}
}
