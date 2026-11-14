//! Answering that there is no console
//!
//! `Offline` is `Client` for a project with no connection: each operation
//! sends nothing and answers that the project is not connected.

use std::future::Future;
use std::pin::Pin;

use crate::console::{client::Client, error::Error, offline::Offline};
use crate::contract::{offer::Offer, reply::Reply, report::Report};

impl Client for Offline {
	fn report<'call>(
		&'call self,
		_report: &'call Report,
	) -> Pin<Box<dyn Future<Output = Result<Reply, Error>> + 'call>> {
		Box::pin(async { Err(Error::Offline) })
	}

	fn offer<'call>(
		&'call self,
	) -> Pin<Box<dyn Future<Output = Result<Offer, Error>> + 'call>> {
		Box::pin(async { Err(Error::Offline) })
	}
}
