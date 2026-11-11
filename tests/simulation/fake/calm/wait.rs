//! A wait that never ends

use std::future::Future;
use std::pin::Pin;

use nsx::session::interrupt::Interrupt;

use crate::simulation::fake::calm::Calm;

impl Interrupt for Calm {
	fn wait(&self) -> Pin<Box<dyn Future<Output = ()> + '_>> {
		Box::pin(std::future::pending())
	}
}
