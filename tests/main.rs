//! The tests of nsx, as one target
//!
//! One folder per area of `src/`, each with a `mod.rs` that declares its
//! files. They build into a single test program, so a fake or a fixture is
//! written once and every test reaches it through `crate::`, with no path
//! that climbs and nothing of the tests under `src/` (Rust rules, item 1).

mod checks;
mod command;
mod components;
mod console;
mod contract;
mod delivery;
mod driver;
mod evidence;
mod files;
mod flow;
mod handoff;
mod harness;
mod hook;
mod judgment;
mod local;
mod money;
mod process;
mod rag;
mod request;
mod runtime;
mod seat;
mod session;
mod simulation;
mod skills;
