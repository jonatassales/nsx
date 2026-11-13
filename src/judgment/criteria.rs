//! The judgment of the guardrail's criteria
//!
//! The judge's first question: what each criterion of the guardrail asks, in a
//! few words. The statements let the team see that the plan understood its
//! rules.
//!
//! Definition only: the `Criteria` type: per guardrail criterion, its id and
//! the judge's statement. Its functions are the files in `criteria/`.
//!
//! ## Simulation
//!
//! - Card: Analysis · the FIN line, which `harness/` shows from these
//!   statements
//!
//! ## Rules
//!
//! - Every criterion is judged, in the guardrail's order: none is skipped and
//!   none is invented.

pub mod ask;
pub mod card;
pub mod line;

use crate::contract::criterion::Criterion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Criteria {
	pub stated: Vec<Criterion>,
}
