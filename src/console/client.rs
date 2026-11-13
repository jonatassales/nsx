//! The console, as nsx reaches it
//!
//! The team's console, reached through one tool: `langdrift.runtime.report`,
//! called over MCP. A call carries the runtime's id, the run's status and the
//! hand-off records written so far. The answer carries the runtime's id and
//! the request the team made and nsx has not applied yet, or none.
//!
//! Definition only: the trait `Client`, which makes one report call and gives
//! back the console's answer, so that tests can replace it with a fake.
//!
//! ## Rules
//!
//! - It carries the one tool and no other.
//! - A call prints no line (the owner's exception to ADR 003).
//! - The answer carries the runtime's id and, when the team asked something
//!   nsx has not applied yet, that request.

use std::future::Future;
use std::pin::Pin;

use crate::console::error::Error;
use crate::contract::{reply::Reply, report::Report};

pub const TOOL: &str = "langdrift.runtime.report";

pub trait Client {
	fn report<'call>(
		&'call self,
		report: &'call Report,
	) -> Pin<Box<dyn Future<Output = Result<Reply, Error>> + 'call>>;
}
