//! A plan without the team's Anthropic key
//!
//! The console's judge calls Anthropic with the key the team saved on the
//! console's Settings → AI Models. Without it the judge is never asked: the
//! plan stops where its first question would be, says why and where to add
//! the key, and writes no draft. Nothing is paid for.
//!
//! Definition only: the `Keyless` type, which carries nothing: the stop is
//! the same for every plan. Its functions are the files in `keyless/`.
//!
//! ## Simulation
//!
//! - The plan's lines up to `guardrail`, then `judgeStopped` and
//!   `planStopped` in place of `judgeGuardrail` and everything after it
//! - Card: Analysis · header `judge unavailable · no Anthropic key`, and the
//!   JUDGE field `no key · Settings → AI Models`
//!
//! ## Rules
//!
//! - The two lines print, the card records the judge as unavailable, and
//!   the stop rule follows: the plan ends with the judge's sentence on
//!   standard error and no draft.

pub mod card;
pub mod line;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keyless;
