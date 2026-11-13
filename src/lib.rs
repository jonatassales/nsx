//! nsx, the LangDrift CLI
//!
//! nsx makes real the session the website's home plays: the simulation. It
//! has three commands. `nsx init` creates the `.langdrift/` folder in a team's
//! project. `nsx agent plan <ticket>` reads the ticket and proposes a sized
//! `runtime.md` draft to the team, with nothing built, started or paid for.
//! `nsx agent run` applies the intent the team authorized. ADR 003 defines
//! the product: nsx does exactly what the simulation shows, nothing more and
//! nothing less.
//!
//! A folder belongs to one element of the simulation:
//!
//! - `command`: the three commands, each as its steps in order
//! - `session`: a printed line, its four kinds and the stop rule
//! - `contract`: every shape a file or the console sees
//! - `handoff`: `plan.json` and the intent the team authorizes
//! - `evidence`: the run's id, `run.json` and the usage record
//! - `request`: the ticket and the documents it links
//! - `harness`: the guardrail and the bound on cpu and memory
//! - `money`: the estimate, the gate and what is released
//! - `rag`: the project's episodes and the search over them
//! - `judgment`: the judge and its three questions
//! - `components`: the client's components, inspected
//! - `runtime`: the sizing and the `runtime.md` draft
//! - `flow`: the stages and the checkpoints a person confirms
//! - `driver`: the coding agent, called once per stage
//! - `skills`: the skills and the report each one writes
//! - `hook`: the effort, recorded before implement may start
//! - `seat`: the part a person writes, and Safe Seat
//! - `local`: the cluster on the person's machine
//! - `delivery`: the branch, the push and the pull request
//! - `checks`: the client's CI checks
//! - `console`: the report to the console, Play and Pause
//! - `process`: running other programs (shared)
//! - `files`: the project's files (shared)

#![deny(clippy::unwrap_used, clippy::expect_used)]

pub mod checks;
pub mod command;
pub mod components;
pub mod console;
pub mod contract;
pub mod delivery;
pub mod driver;
pub mod evidence;
pub mod files;
pub mod flow;
pub mod handoff;
pub mod harness;
pub mod hook;
pub mod judgment;
pub mod local;
pub mod money;
pub mod process;
pub mod rag;
pub mod request;
pub mod runtime;
pub mod seat;
pub mod session;
pub mod skills;
