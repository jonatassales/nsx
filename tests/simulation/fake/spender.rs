//! The person who answers each wait, while a call is paid behind their back
//!
//! Once the usage record exists, every answer adds one more call to its
//! first run before it is given: the run that waits finds it paid meanwhile.

mod line;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Spender {
	pub file: PathBuf,
}
