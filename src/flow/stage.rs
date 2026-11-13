//! One stage of a run
//!
//! A stage of the workflow as this run plays it: whether a person confirms
//! it, the state it has reached and who confirmed it. The run records the
//! state at each line that changes it, and the card shows what was recorded.
//!
//! Definition only: the `Stage` type, which carries the stage's `id`, `human`
//! (ticked or not), `state` (`pending`, `waiting`, `active` or `done`) and
//! `confirmedBy` (`owner` once confirmed).
//!
//! ## Simulation
//!
//! - Card: Workflow · HUMAN IN THE LOOP, the stage states: pending until the
//!   stage's first line prints, active while its lines print, done after its
//!   last. A ticked stage reads `waiting for the owner` while it is active.
//!
//! ## Rules
//!
//! - The states go into the run's record.
//! - A stage that fails keeps the state it had. There is no failed state.
//! - A run paused at the gate leaves every stage pending.
//! - The Dev ticket's states are `seat`'s.
