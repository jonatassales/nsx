//! The one file comes back, whatever is asked

use std::future::Future;
use std::pin::Pin;

use nsx::local::download::Download;

use crate::simulation::fake::shelf::Shelf;

impl Download for Shelf {
	fn get<'call>(
		&'call self,
		_url: &'call str,
	) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, String>> + 'call>> {
		self.asked.set(self.asked.get() + 1);

		Box::pin(async { Ok(self.bytes.clone()) })
	}
}
