//! The retrieval's cited line
//!
//! One function: it gives back the line that says how many episodes the plan
//! cites and that none of them measured resources, or refuses when one did.
//!
//! ## Simulation
//!
//! - `episodes`: `console › 3 episodes cited · none with measured resources`
//! - Card: Analysis · MEASURED RESOURCES: `none cited`
//!
//! ## Rules
//!
//! - "none with measured resources" is shown when no cited episode measured
//!   cpu or memory. A missing measurement never reads as zero.
//! - The count of cited episodes that measured resources is the input of the
//!   judge's `reasonResources` line, whose last words are added beside
//!   `none cited`.
//!
//! ## Open
//!
//! - A cited episode with measured resources: the simulation shows no wording
//!   for it, so this line and MEASURED RESOURCES have none until it does.
//!   Until then such a plan stops here, with one sentence.
//! - Fewer candidates, or none: until the owner adds the case to the
//!   simulation, the line keeps its template with the real count.

use crate::rag::{error::Error, retrieval::Retrieval};
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Retrieval {
	pub fn cited(&self) -> Result<Line, Error> {
		if self.cited_measured > 0 {
			return Err(Error::Measured {
				cited: self.cited_measured,
			});
		}

		Ok(Line {
			id: String::from("episodes"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!(
					"{} episodes cited · none with measured resources",
					self.cited
				),
			},
		})
	}
}
