//! The judge's reasoning
//!
//! The plan's reasoning step. The judge answers three questions: what each
//! guardrail criterion asks, why each episode candidate is kept or differs,
//! and which model and effort the ticket warrants. Each answer is checked
//! against fixed rules before any line prints. The judge only recommends: it
//! never decides.
//!
//! - `Judge`: the judge, as a trait
//! - `Question`: one question for the judge, with what the judge gets
//! - `Absent`: no judge, which refuses every question
//! - `Error`: what asking the judge can refuse
//! - `Client`: the judge reached over HTTP
//! - `Criteria`: judging the guardrail's criteria
//! - `Relevance`: judging which episodes are relevant
//! - `Suggestion`: the suggested model and effort, and its four lines
//!
//! ## Simulation
//!
//! - Card: Analysis · header, in three states: `4 documents read` from the
//!   `reads` line, `3 of 6 episodes kept` from the `drop` line, and
//!   `Claude Sonnet 5.5 · effort medium` from the `suggest` line on
//!
//! ## Rules
//!
//! - Nothing about the judge is priced, reserved or gated in the plan.
//! - The six judged lines are the only lines about the judge: no warning line
//!   and no mode without a judge exist.
//! - When the judge cannot be reached, declines, or gives an answer the checks
//!   refuse, the stop rule applies: the plan stops before the line the answer
//!   would feed and writes no draft.
//!
//! Spec 0078.

pub mod absent;
pub mod client;
pub mod criteria;
pub mod error;
pub mod judge;
pub mod question;
pub mod relevance;
pub mod suggestion;
