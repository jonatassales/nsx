//! The checkpoint's lines
//!
//! One function: it gives back the line a checkpoint prints in the state it
//! is in: the `waiting` line before the stage's work, or the `confirmed`
//! line once the owner has answered.
//!
//! ## Simulation
//!
//! - `waiting-planning`: `console › waiting    planning · the owner confirms`
//! - `confirmed-planning`: `✓ confirmed  planning · by the owner`
//! - `waiting-spec`: `console › waiting    spec · the owner confirms`
//! - `confirmed-spec`: `✓ confirmed  spec · by the owner`
//! - `waiting-tickets`: `console › waiting    tickets · the owner confirms`
//! - `confirmed-tickets`: `✓ confirmed  tickets · by the owner`
//! - `waiting-implement`: `console › waiting    implement · the owner confirms`
//! - `confirmed-implement`: `✓ confirmed  implement · by the owner`
//!
//! ## Rules
//!
//! - Planning's pair prints in every run that passes the gate. The other
//!   pairs print only when their stage is ticked.
//! - A pair prints before its stage's own work: Planning's before `grilling`,
//!   Spec's before `to-spec`, Tickets' before `to-tickets`, Implement's
//!   before the `hook` line.
//! - Nothing prints between the two lines of a pair.
//! - The lines carry the stage's id, not the card's label.

use crate::flow::checkpoint::Checkpoint;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Checkpoint {
	pub fn line(&self) -> Line {
		match self {
			Self::Waiting {
				stage,
			} => Line {
				id: format!("waiting-{stage}"),
				kind: Kind::Output,
				shape: Shape::Labeled {
					label: String::from("waiting"),
					value: format!("{stage} · the owner confirms"),
				},
			},
			Self::Confirmed {
				stage,
			} => Line {
				id: format!("confirmed-{stage}"),
				kind: Kind::Success,
				shape: Shape::Result {
					label: String::from("confirmed"),
					value: format!("{stage} · by the owner"),
				},
			},
		}
	}
}
