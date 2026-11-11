//! One stage's paid call
//!
//! One function: it takes what the run applies, the run's record, what the
//! stages share and one stage of the workflow, and plays that stage's work:
//! the call to the coding agent, the usage record, the report's check and
//! the stage's one line.
//!
//! The steps:
//!
//! - hook: for Implement, ask whether the recorded effort lets it start
//! - driver: start the coding agent, once
//! - evidence: keep what the call used, whatever its end
//! - driver: keep what the agent printed, in the call's log
//! - driver: refuse a call that failed or reported no tokens
//! - skills: check the stage's report and print its line
//! - evidence: record the stage's figures, and the stage as done
//!
//! ## Rules
//!
//! - The usage record keeps the call before anything can stop the run, so
//!   no spend goes unrecorded.
//! - A stage with no skill makes no call.
//!
//! Spec 0077.

use crate::command::{
	applied::Applied, run::Run, session::Session, work::Work,
};
use crate::contract::step::DONE;
use crate::driver::call::Call;
use crate::evidence::record::Record;
use crate::flow::entry::Entry;
use crate::hook::effort::Effort;
use crate::session::stop::Stop;
use crate::skills::report::Report;

impl Run {
	pub async fn call(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
		record: &mut Record,
		work: &mut Work,
		entry: &Entry,
	) -> Result<(), Stop> {
		let root = &applied.root;
		let intent = &applied.authorization.intent;
		let stage = entry.id.as_str();
		let Some(skill) = &entry.skill else {
			return Ok(());
		};

		let run = applied.id.text();
		let call = Call {
			root: root.path.clone(),
			run: run.clone(),
			ticket: intent.ticket.clone(),
			stage: String::from(stage),
			skill: skill.clone(),
			model: intent.model.clone(),
			effort: intent.effort.clone(),
			report: applied
				.layout
				.runs
				.join(run)
				.join("stages")
				.join(format!("{stage}.report.json")),
		};

		if stage == "implement" {
			Effort::permit(work.effort.as_ref(), &intent.model, &intent.effort)?;
		}

		let ended = call.run(session.agent, &work.config, root).await?;

		// Both are tried before either failure stops the run: a call that was
		// paid is never left with neither its usage nor its log.
		let kept = work.usage.write(root, &work.file, &ended);
		let logged = call.log(root, &ended);

		kept?;
		logged?;

		ended.accept()?;

		let report = Report::check(root, stage, &call.report, &work.spec.path)?;

		session.sink.print(&report.line(&work.spec.short_id))?;
		record
			.stage(root, session.console, stage, |step| {
				step.report = Some(report.figures.clone());
				step.state = String::from(DONE);
			})
			.await?;

		Ok(())
	}
}
