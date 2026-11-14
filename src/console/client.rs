//! The console, as nsx reaches it
//!
//! The team's console, reached through two tools called over MCP.
//! `langdrift.runtime.report` carries the runtime's id, the run's status and
//! the hand-off records written so far, and its answer carries the runtime's
//! id and the request the team made and nsx has not applied yet, or none.
//! `langdrift.account.offer` takes no argument, and its answer is the team's
//! offer: per provider, the models and the efforts set on the console's
//! Settings → AI Models.
//!
//! Definition only: the trait `Client`, which makes one report call or reads
//! the team's offer and gives back the console's answer, so that tests can
//! replace it with a fake. The two tools' names sit beside it.
//!
//! ## Rules
//!
//! - It carries these two tools and no other.
//! - A call prints no line (the owner's exception to ADR 001).
//! - The report's answer carries the runtime's id and, when the team asked
//!   something nsx has not applied yet, that request.

use std::future::Future;
use std::pin::Pin;

use crate::console::error::Error;
use crate::contract::{offer::Offer, reply::Reply, report::Report};

pub const REPORT: &str = "langdrift.runtime.report";

pub const OFFER: &str = "langdrift.account.offer";

pub trait Client {
	fn report<'call>(
		&'call self,
		report: &'call Report,
	) -> Pin<Box<dyn Future<Output = Result<Reply, Error>> + 'call>>;

	fn offer<'call>(
		&'call self,
	) -> Pin<Box<dyn Future<Output = Result<Offer, Error>> + 'call>>;
}
