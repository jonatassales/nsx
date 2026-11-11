//! Reaching the Safe Seat verdict
//!
//! One function: it takes the dev ticket as the run closed it and the clock
//! and, once the client's checks have passed, gives back the verdict.
//!
//! ## Rules
//!
//! - Safe Seat is met when the run's dev ticket was closed by the person
//!   with no paid call during it. A count that is unknown is not zero: it
//!   does not meet it.
//! - It is decided after `✓ checks     4 of 4 passed` has printed (spec
//!   0064). A run that did not reach the checks gets no verdict at all.
//!
//! ## Open
//!
//! - How "met" is decided. The simulation never shows the measure. The rule
//!   above holds until the owner decides. A floor measured on the diff, the
//!   checks' result, or both may be wanted.
//!
//! Spec 0080.

use crate::contract::step::DONE;
use crate::evidence::clock::Clock;
use crate::seat::{
	part::{PERSON, Part},
	verdict::Verdict,
};

impl Verdict {
	pub fn reach(part: &Part, clock: &dyn Clock) -> Self {
		let closed =
			part.state == DONE && part.closed_by.as_deref() == Some(PERSON);
		let unpaid = part.paid_calls == Some(0);

		Self {
			met: closed && unpaid,
			decided_at: clock.now(),
		}
	}
}
