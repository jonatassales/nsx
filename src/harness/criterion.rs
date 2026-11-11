//! One criterion of a guardrail
//!
//! One row of a guardrail's criteria table: a rule the team wrote down, such
//! as FIN-02, and the steps it is checked at. The plan reads it. nsx never
//! enforces it.
//!
//! Definition only: the `Criterion` type: its id, its rule text and the steps
//! it is checked at, the plan, the run or both.
//!
//! ## Rules
//!
//! - The rule text goes to the judge, which returns one short statement per
//!   criterion. What a criterion requires is the judge's to state.
//! - Criteria keep the table's order.
//!
//! ## Open
//!
//! - Whether this type also carries the judge's short statement, which the
//!   card's FIN line shows, or only what the document holds.
//!
//! Spec 0078.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Criterion {
	pub id: String,
	pub rule: String,
	pub plan: bool,
	pub run: bool,
}
