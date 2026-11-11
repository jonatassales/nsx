//! Answering that there is no console
//!
//! One function: it takes a report, sends nothing and answers that the
//! project is not connected.
//!
//! Spec 0081.

use std::future::Future;
use std::pin::Pin;

use crate::console::{client::Client, error::Error, offline::Offline};
use crate::contract::{reply::Reply, report::Report};

impl Client for Offline {
	fn report<'call>(
		&'call self,
		_report: &'call Report,
	) -> Pin<Box<dyn Future<Output = Result<Reply, Error>> + 'call>> {
		Box::pin(async { Err(Error::Offline) })
	}
}
