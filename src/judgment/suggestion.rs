//! The suggestion
//!
//! The judge's third question: which model and effort the ticket warrants,
//! with one reason against the efforts and one against the models it did not
//! pick. The suggestion is a recommendation: the model and the effort become
//! the team's only when the team authorizes them.
//!
//! Definition only: the `Suggestion` type: the model's id, the effort, the
//! effort reason and the model reason. It always comes as a recommendation,
//! so that is not kept.
//! Its functions are the files in `suggestion/`.
//!
//! ## Simulation
//!
//! - Card: Analysis · SUGGESTED MODEL and SUGGESTED EFFORT: placeholders from
//!   the `memory` line, then their values from the `suggest` line
//! - Card: Analysis · `recommendation · the team authorizes`
//!
//! ## Rules
//!
//! - The model is one of the three of the price list, the ones the Workflow
//!   card offers. The effort is low, medium or high.
//! - It always comes from the judge: no recorded choice stands in for it.
//! - It goes into the plan's card data and the `runtime.md` draft as a
//!   recommendation, never as an authorization.
//! - It does not set the Workflow card's default choice.

pub mod ask;
pub mod card;
pub mod effort;
pub mod line;
pub mod model;
pub mod resources;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
	pub model: String,
	pub effort: String,
	pub effort_reason: String,
	pub model_reason: String,
}
