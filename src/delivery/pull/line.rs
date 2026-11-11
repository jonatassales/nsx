//! The pull request's lines
//!
//! One function: it takes the pull request and what the run released, and
//! gives back the line the pull request prints, which is the pr line while
//! it is a draft, then the ready line once it is marked ready.
//!
//! ## Simulation
//!
//! - `pullRequest`: `✓ pr #19     feat(demo): run book a demo as a microruntime`
//! - `ready`: `✓ ready      #19 for review · $1.60 released to the envelope`
//! - `ready`, with Review unticked: `✓ ready      #19 to merge · $1.60 released to the envelope`
//!
//! ## Rules
//!
//! - `for review` or `to merge` follows spec 0057's `reviewed` and nothing
//!   else.
//! - The released amount inside `ready` is supplied by `money/settlement`
//!   (spec 0056). It is the run's reserved minus observed, so it changes with
//!   the model and the effort. The pr line does not show it.
//! - The run's `# next` comment follows the ready line (spec 0057).
//! - Neither line prints in an over-budget run.
//!
//! Spec 0081.

use crate::delivery::{pull::Pull, readiness::Readiness};
use crate::money::cents::Cents;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Pull {
	pub fn line(&self, released: Cents) -> Line {
		let aim = match self.state {
			Readiness::Draft => {
				return Line {
					id: String::from("pullRequest"),
					kind: Kind::Success,
					shape: Shape::Result {
						label: format!("pr #{}", self.number),
						value: self.title.clone(),
					},
				};
			}
			Readiness::Review => "for review",
			Readiness::Merge => "to merge",
		};

		Line {
			id: String::from("ready"),
			kind: Kind::Success,
			shape: Shape::Result {
				label: String::from("ready"),
				value: format!(
					"#{} {aim} · {} released to the envelope",
					self.number,
					released.format()
				),
			},
		}
	}
}
