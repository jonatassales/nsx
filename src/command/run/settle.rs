//! Closing the implementation's money
//!
//! One function: it takes what the run applies, the run's money entry and
//! the usage record the stages filled, prices what the run used and prints
//! the line that closes the implementation.
//!
//! The steps:
//!
//! - money: price the observed tokens at the applied model's list price
//! - evidence: the completed line (`completed`)
//! - money: write the observed tokens and spend to the ledger
//!
//! ## Rules
//!
//! - A run whose usage is unknown stops before this line: it never prints a
//!   figure that was not measured.
//! - It runs after the `runtimeReady` line and before the push.
//! - The release is written when the `ready` line prints (`carry`).

use crate::command::{applied::Applied, run::Run, session::Session};
use crate::evidence::{error::Error as Evidence, usage::Usage};
use crate::money::{
	cents::Cents, error::Error, ledger::Ledger, settlement::Settlement,
};
use crate::session::stop::Stop;

impl Run {
	pub fn settle(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
		ledger: &mut Ledger,
		usage: &Usage,
	) -> Result<Settlement, Stop> {
		let intent = &applied.authorization.intent;
		let unpriced = || Error::Unpriced {
			model: intent.model.clone(),
			effort: intent.effort.clone(),
		};
		let input = usage.input_tokens.ok_or(Evidence::Unmeasured)?;
		let output = usage.output_tokens.ok_or(Evidence::Unmeasured)?;
		let listed = applied
			.prices
			.models
			.iter()
			.find(|model| model.id == intent.model);
		let model = listed.ok_or_else(unpriced)?;
		let reserved = Cents(ledger.reserved_cents);
		let settlement = Settlement::settle(reserved, model, input, output)
			.ok_or_else(unpriced)?;

		session
			.sink
			.print(&Usage::line(settlement.observed, reserved))?;

		ledger.observed_input_tokens = Some(settlement.input_tokens);
		ledger.observed_output_tokens = Some(settlement.output_tokens);
		ledger.observed_cents = Some(settlement.observed.0);
		ledger.write(&applied.root, &applied.layout)?;

		Ok(settlement)
	}
}
