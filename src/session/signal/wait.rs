//! Waiting for the system's interrupt
//!
//! One function: it waits until one of the three watched signals arrives.
//!
//! ## Rules
//!
//! - A signal that arrived since the last wait ended this one at once.
//!   Several that arrive together count as one.

use std::future::Future;
use std::pin::Pin;

use crate::session::{interrupt::Interrupt, signal::Signal};

impl Interrupt for Signal {
	fn wait(&self) -> Pin<Box<dyn Future<Output = ()> + '_>> {
		Box::pin(async move {
			let mut streams = self.streams.lock().await;
			let [interrupt, end, hangup] = &mut *streams;

			tokio::select! {
				_interrupted = interrupt.recv() => {}
				_ended = end.recv() => {}
				_closed = hangup.recv() => {}
			}
		})
	}
}
