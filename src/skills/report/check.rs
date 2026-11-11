//! Checking a stage's report
//!
//! One function: it takes the project's root, the stage, the report path and
//! the spec the ticket links, reads the report, and gives back the stage's
//! report, checked, or the reason the stage fails.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/agent/state/runs/<run>/stages/<stage>.report.json`
//!
//! ## Rules
//!
//! - The call's exit status and its tokens are the driver's to refuse, before
//!   the report is read.
//! - A report that is missing, is not JSON, lacks a field, or holds false
//!   where the line prints fixed text fails the stage. So does a method other
//!   than `tdd`, or a spec other than the one the ticket links.
//! - A failed stage prints no line and stops the run (the stop rule).
//! - The report file is private and ignored. Only its figures are kept.
//!
//! ## Open
//!
//! - Outcomes the simulation does not show: findings left unfixed, an
//!   implement that went past the boundary, a spec written rather than
//!   revised, tickets without blocking edges. Until the owner decides, such a
//!   report fails the stage.
//!
//! Spec 0080.

use std::path::Path;

use serde::de::DeserializeOwned;

use crate::contract::{decode::decode, figures::Figures};
use crate::files::{
	inside::inside,
	root::Root,
	text::{DOCUMENT, text},
};
use crate::skills::{error::Error, report::Report};

impl Report {
	pub fn check(
		root: &Root,
		stage: &str,
		path: &Path,
		spec: &str,
	) -> Result<Self, Error> {
		// The report of one stage, read strictly in that stage's own shape.
		fn read<Shape: DeserializeOwned>(
			stage: &str,
			written: &str,
		) -> Result<Shape, Error> {
			decode(written).map_err(|cause| Error::Unread {
				stage: String::from(stage),
				cause,
			})
		}

		let file = inside(root, path)?;

		if !file.exists() {
			return Err(Error::Missing {
				stage: String::from(stage),
				path: file,
			});
		}

		let written = text(root, path, DOCUMENT)?;
		let refused = |field: &'static str, rule: &str| Error::Refused {
			stage: String::from(stage),
			field,
			rule: String::from(rule),
		};
		// Where a line prints fixed words, the report must say them true.
		let holds = |field: &'static str, said: bool| {
			if said {
				return Ok(());
			}

			Err(refused(field, "must be true"))
		};
		let figures = match stage {
			"planning" => Figures::Planning(read(stage, &written)?),
			"spec" => {
				let report = read::<crate::contract::spec::Spec>(stage, &written)?;

				holds("revised", report.revised)?;

				if report.spec != spec {
					return Err(refused(
						"spec",
						&format!("must be the spec the ticket links, {spec}"),
					));
				}

				Figures::Spec(report)
			}
			"tickets" => {
				let report =
					read::<crate::contract::tickets::Tickets>(stage, &written)?;

				holds("blockingEdgesRecorded", report.blocking_edges_recorded)?;

				Figures::Tickets(report)
			}
			"implement" => {
				let report =
					read::<crate::contract::implement::Implement>(stage, &written)?;

				if report.method != "tdd" {
					return Err(refused("method", "must be tdd"));
				}

				holds("boundaryOnly", report.boundary_only)?;
				holds("stoppedAtReservedPart", report.stopped_at_reserved_part)?;

				Figures::Implement(report)
			}
			"review" => {
				let report = read::<crate::contract::review::Review>(stage, &written)?;

				holds("fixed", report.fixed)?;

				Figures::Review(report)
			}
			_ => {
				return Err(Error::Unknown {
					stage: String::from(stage),
				});
			}
		};

		Ok(Self {
			stage: String::from(stage),
			figures,
		})
	}
}
