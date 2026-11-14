//! No file comes back, whatever is asked

use std::future::Future;
use std::pin::Pin;

use nsx::local::download::Download;

use crate::simulation::fake::unreached::Unreached;

impl Download for Unreached {
	fn get<'call>(
		&'call self,
		_url: &'call str,
	) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, String>> + 'call>> {
		Box::pin(async { Err(String::from("the web is not reached in tests")) })
	}
}
