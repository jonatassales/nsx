//! The analysis act
//!
//! The second act of the plan: the linked documents are read, the bound is
//! stated, the episodes are searched, and a model and an effort are suggested.
//!
//! One function: it takes what the plan read before its first line and the
//! card data's writer, and plays the act, each step calling the folder that
//! owns the line. It gives back the spec it read and the judge's suggestion,
//! for the plan act and the draft.
//!
//! The steps, in the simulation's order:
//!
//! - here: the `analysis` comment
//! - request: read the PRD, the design doc and the spec (`readPrd`,
//!   `readDesignDoc`, `readSpec`), then say what was read (`reads`)
//! - harness: read the guardrail (`guardrail`)
//! - judgment: state the guardrail's criteria (`judgeGuardrail`)
//! - harness: state the bound (`bounds`)
//! - money: state the budget per run (`budget`)
//! - rag: embed the ticket's intent and search the episodes (`memory`,
//!   `embed`, `vectorSearch`, `candidates`)
//! - judgment: give each candidate its reason (`judgeRelevance`)
//! - rag: apply the threshold and count what is cited (`drop`, `episodes`)
//! - judgment: state the sizing basis, weigh the efforts and the models, and
//!   suggest (`reasonResources`, `weighEffort`, `weighModel`, `suggest`)
//!
//! ## Simulation
//!
//! - `analysis`: `# analysis · reading the linked documents, then the project's episodes`
//!
//! ## Rules
//!
//! - The embedder is local, the judge is neither priced nor gated, and
//!   nothing is reserved: the act keeps the promise of the plan's first line.

use crate::command::{brief::Brief, desk::Desk, plan::Plan, session::Session};
use crate::harness::{envelope::Envelope, guardrail::Guardrail};
use crate::judgment::{
	criteria::Criteria, relevance::Relevance, suggestion::Suggestion,
};
use crate::money::budget::Budget;
use crate::rag::retrieval::Retrieval;
use crate::request::{
	documents::Documents, prd::Prd, reading::Reading, spec::Spec,
};
use crate::session::{line::Line, stop::Stop};

impl Plan {
	pub async fn analyze(
		&self,
		session: &mut Session<'_>,
		brief: &Brief,
		desk: &mut Desk,
	) -> Result<(Spec, Suggestion), Stop> {
		let root = &brief.root;
		let design = &brief.design;

		session.sink.print(&Line::comment(
			"analysis",
			"analysis · reading the linked documents, then the project's episodes",
		))?;

		let prd = Prd::read(root, &brief.links.prd)?;

		session.sink.print(&prd.line())?;
		desk
			.record(root, session.console, |cards| prd.card(cards))
			.await?;

		session.sink.print(&design.line())?;
		desk
			.record(root, session.console, |cards| design.card(cards))
			.await?;

		let spec = Spec::read(root, &brief.links.spec)?;

		session.sink.print(&spec.line())?;
		desk
			.record(root, session.console, |cards| spec.card(cards))
			.await?;

		let reading = Reading::count(&brief.links);

		session.sink.print(&reading.line())?;
		desk
			.record(root, session.console, |cards| reading.card(cards))
			.await?;

		let guardrail = Guardrail::load(root, &brief.links.guardrail)?;

		session.sink.print(&guardrail.line())?;
		desk
			.record(root, session.console, |cards| guardrail.card(cards))
			.await?;

		let criteria = Criteria::ask(session.judge, &guardrail).await?;

		session.sink.print(&criteria.line())?;
		desk
			.record(root, session.console, |cards| criteria.card(cards))
			.await?;

		let envelope = Envelope::of(design);
		let budget = Budget::of(design);

		session.sink.print(&envelope.line())?;
		desk
			.record(root, session.console, |cards| envelope.card(cards))
			.await?;

		session.sink.print(&budget.line())?;

		session.sink.print(&Retrieval::memory())?;

		let intent = brief.ticket.intent(&brief.written);
		let mut retrieval = Retrieval::retrieve(root, &brief.layout, &intent)?;

		session.sink.print(&Retrieval::embedding())?;

		session.sink.print(&Retrieval::search())?;

		session.sink.print(&retrieval.candidates())?;
		desk
			.record(root, session.console, |cards| retrieval.rows(cards))
			.await?;

		let documents = Documents {
			prd: &prd,
			design,
			spec: &spec,
		};
		let relevance =
			Relevance::ask(session.judge, &retrieval, documents).await?;

		relevance.explain(&mut retrieval);

		session.sink.print(&relevance.line())?;

		session.sink.print(&retrieval.dropped())?;
		desk
			.record(root, session.console, |cards| retrieval.verdicts(cards))
			.await?;

		let cited = retrieval.cited()?;

		session.sink.print(&cited)?;
		desk
			.record(root, session.console, |cards| retrieval.citations(cards))
			.await?;

		session.sink.print(&Suggestion::resources())?;

		let suggestion =
			Suggestion::ask(session.judge, &retrieval, documents, &brief.prices)
				.await?;

		session.sink.print(&suggestion.effort())?;

		session.sink.print(&suggestion.model())?;

		session.sink.print(&suggestion.line())?;
		desk
			.record(root, session.console, |cards| suggestion.card(cards))
			.await?;

		Ok((spec, suggestion))
	}
}
