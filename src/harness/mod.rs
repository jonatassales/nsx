//! The guardrail and the envelope
//!
//! The harness is the part of the client's `.langdrift/` folder that holds the
//! team's guardrails. Here the plan reads the guardrail the ticket links,
//! names the criteria the plan step answers to, and states the design doc's
//! envelope as the plan's bound.
//!
//! - `Guardrail`: the guardrail document and its criteria
//! - `Criterion`: one criterion of a guardrail
//! - `Envelope`: the bound on cpu and memory, set by the team
//! - `Error`: what reading the harness can refuse
//!
//! ## Rules
//!
//! - This folder writes no file. The guardrail documents stay where they are,
//!   and `harness/envelope.json` is neither read nor written.
//! - nsx enforces no guardrail criterion. Every guardrail is recorded as a
//!   review rule.
//! - `request/` finds the guardrail link and extracts the envelope. This folder
//!   reads the guardrail and states the bound.
//! - No guardrail link, an unreadable guardrail, no criteria table, a row
//!   lacking `Checked at`, or a design doc without the envelope stops the plan
//!   before the line this folder would print: no further line, no draft, one
//!   sentence on stderr, exit 1 (the stop rule).

pub mod criterion;
pub mod envelope;
pub mod error;
pub mod guardrail;
