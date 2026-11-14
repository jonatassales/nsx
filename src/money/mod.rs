//! A run's money
//!
//! The one money mechanism the simulation shows. The plan states the budget
//! per run and prices every model and effort pair. The run prints the estimate
//! and the budget, reserves the estimate plus a fixed margin, and opens the
//! gate or pauses with nothing paid. At the end it prices what the stages
//! observed and releases the difference.
//!
//! - `Cents`: money, in whole cents
//! - `Price`: the price list, three models at their list prices, each a
//!   `Model`
//! - `Candidates`: the models of the price list the team offers on the
//!   console's Settings → AI Models, each a `Candidate` with its efforts
//! - `Forecast`: the tokens a run is expected to use, per effort
//! - `Budget`: the budget per run, set by the team in the design doc
//! - `Estimate`: what a run is expected to spend
//! - `Reservation`: the estimate plus its margin
//! - `Gate`: opened or paused
//! - `Spend`: the spend table, one row per candidate model and effort
//! - `Ledger`: the run's money record
//! - `Settlement`: what was observed and what was released
//! - `Error`: what reading the money files can refuse
//!
//! ## Rules
//!
//! - All money is whole cents until it is formatted. Unknown is `null`, never
//!   zero.
//! - Nothing is reserved and nothing is paid before the gate opens. Planning
//!   reserves nothing.
//! - The per-run budget is the only bound. No other gate, input or record
//!   stands between the owner and the run.
//! - A missing or unreadable price list or forecast, a design doc without a
//!   budget, or a chosen pair missing from the spend table stops the command
//!   before the line that needs it (the stop rule).
//! - This folder does not read the budget out of the design doc, measure the
//!   tokens the stages use or choose the model and effort.

pub mod budget;
pub mod candidate;
pub mod candidates;
pub mod cents;
pub mod error;
pub mod estimate;
pub mod forecast;
pub mod gate;
pub mod ledger;
pub mod model;
pub mod pair;
pub mod price;
pub mod reservation;
pub mod settlement;
pub mod spend;
pub mod tokens;
