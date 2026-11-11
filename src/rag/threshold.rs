//! The similarity threshold
//!
//! The one rule that decides which candidates the plan cites. A candidate is
//! kept exactly when its similarity, rounded to two decimals, is at least
//! 0.55. Below 0.55 it is dropped.
//!
//! Definition only: the `Threshold` type, which carries the number, and the
//! one threshold nsx uses, 0.55. Its function is the file in `threshold/`.
//!
//! ## Simulation
//!
//! - Card: Analysis · EPISODES header: `threshold 0.55`, shown with the card
//!
//! ## Rules
//!
//! - The comparison is "at least", not "more than": 0.55 is kept and 0.54 is
//!   dropped.
//! - Spec 0052 calls the threshold strict: it is the only rule, and no other
//!   score keeps a candidate below it.
//! - "Kept" always means at or above 0.55.
//!
//! - A similarity is rounded half away from zero, to two decimals, as the Go
//!   CLI rounded it, and the threshold compares the rounded value: the screen
//!   never contradicts the verdict (owner, 2026-11-11).
//!
//! Spec 0078.

pub mod card;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Threshold(pub f64);

pub const THRESHOLD: Threshold = Threshold(0.55);
