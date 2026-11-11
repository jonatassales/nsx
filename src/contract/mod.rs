//! Every shape a file or the console sees
//!
//! The shapes of what nsx writes under `.langdrift/`, reads back and sends to
//! the console: the `runtime.md` fence, the judge's answers, the stage
//! reports, the hand-off records and the report that carries them. Every field
//! feeds a line nsx prints or a field a card shows. The other folders fill
//! these shapes; this one produces no value and touches no file.
//!
//! - `Fence`: the `langdrift` block of `runtime.md`, version 3
//! - `Problem`: one rule a shape breaks, as a field and a rule
//! - `Error`: what reading a shape can refuse
//! - `decode`: reading a record strictly
//! - `plain` and `label`: what a printed text and a runtime's id must be
//! - `schema`: the JSON Schema of a shape, and `nullable` for a required
//!   field that may be `null`
//! - `Cards`: `plan.json`, with `Ticket` and its `Linked` artifacts,
//!   `Analysis`, `Plan` and `Workflow`
//! - `Documents`: the Analysis card's DOCUMENTS, with `Product`, `Design`
//!   `Specified` and `Guardrail`, one row per linked document, with
//!   `Envelope` and `Rule`
//! - `Retrieval`: the Analysis card's EPISODES, one `Candidate` per row
//! - `Recommendation`: the Analysis card's suggested model and effort
//! - `Intent`: `intent.json`, what the team chose, proposed or authorized
//! - `Run` and `Outcome`: `run.json` and how the run ended, with `Hosted`
//!   for its runtime and `Delivery`, `Pull` and `Checks` for what it
//!   delivered
//! - `Report` and `Status`: what nsx sends the console (spec 0073), and
//!   `Reply`: what the console answers
//! - `Judged`, with its `Used` and `Cost`: the console judge's response to
//!   one question
//! - `Criteria`, `Episodes` and `Suggestion`: the judge's three answers, each
//!   with its own `check`
//! - `Stage`: the stages, as the records spell them
//! - `Planning`, `Spec`, `Tickets`, `Implement`, `Review`: the stage reports
//!
//! ## Rules
//!
//! - Figures are whole units: millicores, MiB, cents and tokens. A similarity
//!   is a number with two decimals.
//! - A value that is not known is `null`, never zero.
//! - A field no shape defines is refused.
//! - A section absent from a hand-off record has not been revealed yet.
//! - No shape carries a secret.
//!
//! Spec 0077.

pub mod amount;
pub mod analysis;
pub mod asked;
pub mod authorization;
pub mod bound;
pub mod candidate;
pub mod cards;
pub mod checks;
pub mod choice;
pub mod component;
pub mod components;
pub mod cost;
pub mod criteria;
pub mod criterion;
pub mod decode;
pub mod delivery;
pub mod design;
pub mod documents;
pub mod effort;
pub mod empty;
pub mod encode;
pub mod envelope;
pub mod episode;
pub mod episodes;
pub mod error;
pub mod evidence;
pub mod fence;
pub mod figures;
pub mod guardrail;
pub mod hosted;
pub mod human;
pub mod implement;
pub mod intent;
pub mod judged;
pub mod label;
pub mod linked;
pub mod nullable;
pub mod outcome;
pub mod pair;
pub mod plain;
pub mod plan;
pub mod planning;
pub mod problem;
pub mod product;
pub mod pull;
pub mod recommendation;
pub mod reference;
pub mod reply;
pub mod report;
pub mod requested;
pub mod resolved;
pub mod resource;
pub mod retrieval;
pub mod review;
pub mod rule;
pub mod run;
pub mod schema;
pub mod seat;
pub mod sizing;
pub mod source;
pub mod spec;
pub mod specified;
pub mod stage;
pub mod status;
pub mod step;
pub mod suggestion;
pub mod ticket;
pub mod tickets;
pub mod used;
pub mod validation;
pub mod workflow;
