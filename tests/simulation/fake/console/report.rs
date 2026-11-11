//! Keeping a report, and answering the runtime's id and the next request,
//! unless the console is down

use std::future::Future;
use std::pin::Pin;

use nsx::console::{client::Client, error::Error};
use nsx::contract::{reply::Reply, report::Report};

use crate::simulation::fake::console::{Console, RUNTIME};

impl Client for Console {
	fn report<'call>(
		&'call self,
		report: &'call Report,
	) -> Pin<Box<dyn Future<Output = Result<Reply, Error>> + 'call>> {
		Box::pin(async move {
			self.reports.borrow_mut().push(report.clone());

			let count = self.reports.borrow().len();
			let rewriting = self
				.rewrite
				.borrow()
				.as_ref()
				.filter(|(at, _file, _text)| *at == count)
				.cloned();

			if let Some((_at, file, text)) = rewriting {
				std::fs::write(file, text).unwrap();
			}

			if report.runtime_id.is_some() && report.runtime_id == self.forgets {
				return Err(Error::Unknown);
			}

			let silent = self.silent_from.is_some_and(|from| count >= from);

			if self.down || silent {
				return Err(Error::Unavailable {
					cause: String::from("the console is down"),
				});
			}

			let scripted = self.requests.borrow_mut().pop_front().flatten();
			let requested = scripted.or_else(|| self.pausing(report));

			Ok(Reply {
				runtime_id: String::from(RUNTIME),
				requested,
			})
		})
	}
}
