//! nsx, the LangDrift CLI
//!
//! nsx makes real the session the website's home plays: the simulation. It
//! has three commands. `nsx init` creates the `.langdrift/` folder in a team's
//! project. `nsx agent plan <ticket>` reads the ticket and proposes a sized
//! `runtime.md` draft to the team, with nothing built, started or paid for.
//! `nsx agent run` applies the intent the team authorized. ADR 0005 defines
//! the product: nsx does exactly what the simulation shows, nothing more and
//! nothing less.
//!
//! A folder belongs to one element of the simulation, and to its spec:
//!
//! - `command`: the three commands, each as its steps in order (spec 0048)
//! - `session`: a printed line, its four kinds and the stop rule (spec 0048)
//! - `contract`: every shape a file or the console sees (spec 0049)
//! - `handoff`: `plan.json` and the intent the team authorizes (spec 0050)
//! - `evidence`: the run's id, `run.json` and the usage record (spec 0065)
//! - `request`: the ticket and the documents it links (spec 0053)
//! - `harness`: the guardrail and the bound on cpu and memory (spec 0054)
//! - `money`: the estimate, the gate and what is released (spec 0056)
//! - `rag`: the project's episodes and the search over them (spec 0052)
//! - `judgment`: the judge and its three questions (spec 0051)
//! - `components`: the client's components, inspected (spec 0066)
//! - `runtime`: the sizing and the `runtime.md` draft (spec 0055)
//! - `flow`: the stages and the checkpoints a person confirms (spec 0057)
//! - `driver`: the coding agent, called once per stage (spec 0058)
//! - `skills`: the skills and the report each one writes (spec 0059)
//! - `hook`: the effort, recorded before implement may start (spec 0060)
//! - `seat`: the part a person writes, and Safe Seat (spec 0061)
//! - `local`: the cluster on the person's machine (spec 0062)
//! - `delivery`: the branch, the push and the pull request (spec 0063)
//! - `checks`: the client's CI checks (spec 0064)
//! - `console`: the report to the console, Play and Pause (specs 0073, 0076)
//! - `process`: running other programs (shared)
//! - `files`: the project's files (shared)
//!
//! Specs 0077 to 0081.

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
