//! No answer

use nsx::flow::reader::Reader;

use crate::simulation::fake::silence::Silence;

impl Reader for Silence {
	fn line(&self) -> std::io::Result<Option<String>> {
		Ok(None)
	}
}
