//! The team's Play of a choice, as the console answers it

use nsx::contract::{asked::Asked, choice::Choice, requested::Requested};

pub fn play(choice: &Choice) -> Option<Requested> {
	Some(Requested {
		state: Asked::Play,
		intent: Some(choice.clone()),
	})
}
