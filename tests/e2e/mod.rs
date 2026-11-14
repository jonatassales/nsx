//! The end-to-end run: nsx against a real console, with nobody at the
//! keyboard
//!
//! Every test here is ignored by default: it needs a console, a secret and
//! the web. `tests/e2e/README.md` says how to run it.

mod generated;
mod plan;
mod played;
mod recorded;
