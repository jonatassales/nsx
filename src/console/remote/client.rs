//! The console over the network, as the client nsx reaches
//!
//! `Remote` is `Client` as the console's `/api/mcp` serves it: each
//! operation of the trait calls its tool once, through `call`.
//!
//! ## Rules
//!
//! - The offer is asked with no argument: `arguments` is `{}`.

use std::future::Future;
use std::pin::Pin;

use serde_json::Map;

use crate::console::{
	client::{Client, OFFER, REPORT},
	error::Error,
	remote::Remote,
};
use crate::contract::{offer::Offer, reply::Reply, report::Report};

impl Client for Remote {
	fn report<'call>(
		&'call self,
		report: &'call Report,
	) -> Pin<Box<dyn Future<Output = Result<Reply, Error>> + 'call>> {
		Box::pin(self.call(REPORT, report))
	}

	fn offer<'call>(
		&'call self,
	) -> Pin<Box<dyn Future<Output = Result<Offer, Error>> + 'call>> {
		Box::pin(async move { self.call(OFFER, &Map::new()).await })
	}
}
