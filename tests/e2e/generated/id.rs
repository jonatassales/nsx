//! The generated ticket's bare id, as `nsx agent plan` takes it

use crate::e2e::generated::Generated;

impl Generated {
	pub fn id(&self) -> String {
		format!("{}-TICKET-{}", self.number, self.slug)
	}
}
