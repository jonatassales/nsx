//! The lines not built yet, per golden, each with the spec that builds it
//!
//! A key is a golden line's id. A listed line may differ from the golden or
//! be missing; a listed line that already matches fails the golden test, and
//! so does a key that names no line. Every list is empty since ticket 0431:
//! nsx prints the six goldens whole, and a line that stops matching fails
//! its test.

pub const PLAN: [(&str, &str); 0] = [];

pub const RUN: [(&str, &str); 0] = [];

pub const STAGES: [(&str, &str); 0] = [];

pub const BUDGET: [(&str, &str); 0] = [];

pub const MERGE: [(&str, &str); 0] = [];

pub const HAIKU: [(&str, &str); 0] = [];
