//! The money act of the run
//!
//! Before any paid call, the run states what it expects to spend and what
//! the team allows, then reserves or pauses.
//!
//! One function: it takes what the run applies and the run's money entry, and
//! plays the act, each step calling `money`, which owns the lines. It gives
//! back the gate, opened or paused.
//!
//! The steps, in the simulation's order:
//!
//! - money: the estimate (`estimate`)
//! - money: the budget per run (`budget`)
//! - money: the gate (`gateOpened`), or the pause and its comment
//!   (`gatePaused`, `paused`)
//!
//! ## Rules
//!
//! - The estimate, the reservation and the budget are the plan's, as the
//!   run's opening read them into the money entry (`money/ledger/open.rs`).
//! - The money ledger's entry opens at the `estimate` line and is written
//!   again once the gate is decided.
//! - The gate is decided before any paid call (FIN-02, FIN-03).
//! - A paused gate ends the run with exit code 0. No branch is started, and
//!   no stage call, cluster or forge operation happens.
//!
//! Spec 0077.

use std::path::PathBuf;

use crate::command::{applied::Applied, run::Run, session::Session};
use crate::money::{
	budget::Budget,
	cents::Cents,
	estimate::Estimate,
	gate::Gate,
	ledger::{Ledger, OPENED, PAUSED},
	tokens::Tokens,
};
use crate::session::stop::Stop;

impl Run {
	pub fn reserve(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
		ledger: &mut Ledger,
	) -> Result<Gate, Stop> {
		let estimate = Estimate {
			model: ledger.model.clone(),
			effort: ledger.effort.clone(),
			tokens: Tokens {
				input: ledger.input_thousands,
				output: ledger.output_thousands,
			},
			amount: Cents(ledger.estimate_cents),
		};
		let budget = Budget {
			amount: Cents(ledger.budget_cents),
			source: PathBuf::from(&ledger.budget_source),
		};

		session.sink.print(&estimate.line())?;
		ledger.write(&applied.root, &applied.layout)?;

		session.sink.print(&budget.cite())?;

		let gate = Gate::open(Cents(ledger.reserved_cents), budget.amount);
		let decided = match gate {
			Gate::Opened {
				..
			} => OPENED,
			Gate::Paused {
				..
			} => PAUSED,
		};

		for line in gate.line() {
			session.sink.print(&line)?;
		}

		ledger.gate = Some(String::from(decided));
		ledger.write(&applied.root, &applied.layout)?;

		Ok(gate)
	}
}
