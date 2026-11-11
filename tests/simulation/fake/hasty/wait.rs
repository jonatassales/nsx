//! A wait that ends once the command has had one turn

use std::future::Future;
use std::pin::Pin;
use std::task::Poll;

use nsx::session::interrupt::Interrupt;

use crate::simulation::fake::hasty::Hasty;

impl Interrupt for Hasty {
	fn wait(&self) -> Pin<Box<dyn Future<Output = ()> + '_>> {
		let mut asked = false;

		Box::pin(std::future::poll_fn(move |context| {
			if asked {
				return Poll::Ready(());
			}

			asked = true;
			context.waker().wake_by_ref();

			Poll::Pending
		}))
	}
}
