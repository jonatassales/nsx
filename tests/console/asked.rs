//! What the team asked, with no choice

use nsx::contract::{asked::Asked, requested::Requested};

pub fn asked(state: Asked) -> Option<Requested> {
	Some(Requested {
		state,
		intent: None,
	})
}
