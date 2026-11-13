//! Taking the runtime down when a run stops
//!
//! One function: it takes what the run applies, the run's record and the
//! stop the run met, takes down what the run had brought up, with no line,
//! and gives the stop back, with what the take-down could not do said after
//! it.
//!
//! The steps:
//!
//! - local: read the run's own execution ledger
//! - local: clean up what it lists, then mark the record's runtime stopped,
//!   a write the console is told of
//!
//! ## Rules
//!
//! - Any stop after the bring-up started takes the runtime down, and an
//!   interrupt is a stop. A run that stopped before its bring-up has no
//!   ledger: nothing is asked of the cluster.
//! - Only this run's ledger is cleaned up here. What earlier runs left is
//!   removed before a run's first stage (`prepare`).
//! - The stop is said first. A take-down that fails is said after it, and
//!   once: when the take-down is the stop itself, its sentence is not
//!   repeated.
//! - A second interrupt gives the take-down up, and says so: the next run of
//!   the runtime removes what is left.
//! - It prints no line.

use crate::command::{applied::Applied, run::Run, session::Session};
use crate::evidence::record::Record;
use crate::local::execution::{Execution, FILE};
use crate::session::stop::Stop;

impl Run {
	pub async fn halt(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
		record: &mut Record,
		stop: Stop,
	) -> Stop {
		let root = &applied.root;
		let layout = &applied.layout;
		let file = layout.runs.join(applied.id.text()).join(FILE);
		let interrupt = session.interrupt;
		let cluster = session.cluster;
		let console = session.console;
		let lowered = tokio::select! {
			biased;

			() = interrupt.wait() => Err(Stop {
				sentence: String::from(
					"The runtime's take-down was interrupted too: the next run of this runtime removes what is left.",
				),
			}),
			lowered = async {
				let Some(mut execution) = Execution::load(root, &file)? else {
					return Ok(());
				};

				execution.cleanup(root, layout, cluster).await?;

				record.write(root, console, |run| execution.card(run)).await?;

				Ok::<(), Stop>(())
			} => lowered,
		};

		match lowered {
			Ok(()) => stop,
			Err(left) if left.sentence == stop.sentence => stop,
			Err(left) => Stop {
				sentence: format!("{} {}", stop.sentence, left.sentence),
			},
		}
	}
}
