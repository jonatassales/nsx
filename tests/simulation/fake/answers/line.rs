//! The next answer, or the end of the input when none is left

use nsx::flow::reader::Reader;

use crate::simulation::fake::answers::Answers;

impl Reader for Answers {
	fn line(&self) -> std::io::Result<Option<String>> {
		Ok(self.lines.borrow_mut().pop_front())
	}
}
