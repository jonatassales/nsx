//! The limits line
//!
//! One function: it gives back the warning line the plan prints after the
//! size line: gpu and disk are not `runtime.md` fields, and none was
//! requested.
//!
//! ## Simulation
//!
//! - `limits`: `limits     gpu and disk are not runtime.md fields · none requested`
//! - Card: Plan · GPU · DISK, `not runtime.md fields`
//!
//! ## Rules
//!
//! - The line is a warning by kind. It stops nothing: the plan goes on.

use crate::runtime::limits::Limits;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Limits {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("limits"),
			kind: Kind::Warning,
			shape: Shape::Status {
				label: String::from("limits"),
				value: String::from(
					"gpu and disk are not runtime.md fields · none requested",
				),
			},
		}
	}
}
