//! Calling the console's report tool
//!
//! One function: it takes a report, calls the tool
//! `langdrift.runtime.report` once and gives back the console's answer.
//!
//! ## Reads and writes
//!
//! - posts one JSON-RPC `tools/call` to `<origin>/api/mcp`, with the
//!   protocol version `2025-11-25` and the secret as the bearer
//!
//! ## Rules
//!
//! - The console answers each request on its own, so no `initialize` is
//!   sent first.
//! - A console that answers `unknown-runtime` no longer knows the id sent.
//! - Anything else but an answer that carries the runtime's id is
//!   "unavailable": a refusal, a timeout, a status that is not 200 or a body
//!   that is not the tool's result. The reason is kept without the request's
//!   address.
//!
//! Spec 0081.

use std::future::Future;
use std::pin::Pin;

use serde_json::{Value, json};

use crate::console::{
	client::{Client, TOOL},
	error::Error,
	remote::{PATIENCE, PROTOCOL, Remote},
};
use crate::contract::{reply::Reply, report::Report};

impl Client for Remote {
	fn report<'call>(
		&'call self,
		report: &'call Report,
	) -> Pin<Box<dyn Future<Output = Result<Reply, Error>> + 'call>> {
		Box::pin(async move {
			let unavailable = |cause: String| Error::Unavailable {
				cause,
			};
			let failed =
				|cause: reqwest::Error| unavailable(cause.without_url().to_string());
			let call = json!({
				"jsonrpc": "2.0",
				"id": 1,
				"method": "tools/call",
				"params": {
					"name": TOOL,
					"arguments": report,
				},
			});

			let answered = self
				.http
				.post(format!("{}/api/mcp", self.connection.origin))
				.timeout(PATIENCE)
				.header("mcp-protocol-version", PROTOCOL)
				.bearer_auth(&self.connection.secret)
				.json(&call)
				.send()
				.await
				.map_err(failed)?;

			let status = answered.status();

			let body = answered.json::<Value>().await.map_err(failed)?;

			let refusal = body
				.pointer("/error/message")
				.or_else(|| body.pointer("/error/code"));

			if refusal.and_then(Value::as_str) == Some("unknown-runtime") {
				return Err(Error::Unknown);
			}

			if let Some(refusal) = refusal {
				return Err(unavailable(format!("the console answered {refusal}")));
			}

			let result = body
				.pointer("/result/structuredContent")
				.filter(|_result| status.is_success())
				.cloned()
				.ok_or_else(|| unavailable(format!("the console answered {status}")))?;

			serde_json::from_value(result)
				.map_err(|cause| unavailable(cause.to_string()))
		})
	}
}
