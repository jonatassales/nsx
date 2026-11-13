//! Reading the links out of the ticket
//!
//! One function: it takes the project's root, the ticket and the ticket's
//! text, and gives back its four links, each classified by its file suffix
//! and named by its short id.
//!
//! ## Rules
//!
//! - A ticket links its documents with Markdown links. No other link syntax is
//!   read.
//! - Kinds by suffix: `.prd.md` PRD, `.dd.md` design doc, `.spec.md` spec,
//!   `.gr.md` guardrail. A kind never comes from a model.
//! - Short ids: a name of the form `<number>-<KIND>-<rest>` is shown as
//!   `<number>-<KIND>`, any other name without its suffix.
//! - A PRD, design doc or spec link must stay inside the client's artifact
//!   folder, and the guardrail link inside `.langdrift/harness/guardrails/`.
//! - A link of another kind, or a kind missing or linked twice, stops the plan
//!   (the stop rule).
//! - A link that is, or passes through, a symbolic link is refused. Whether
//!   the document is there is found when it is read.

use std::path::Path;

use pulldown_cmark::{Event, Parser, Tag};

use crate::files::{root::Root, within::within};
use crate::request::{
	error::Error,
	kind::{DESIGN, GUARDRAIL, KINDS, Kind, PRD, SPEC},
	link::Link,
	links::Links,
	ticket::Ticket,
};

impl Links {
	pub fn parse(
		root: &Root,
		ticket: &Ticket,
		text: &str,
	) -> Result<Self, Error> {
		let folder = ticket.path.parent().unwrap_or(Path::new(""));
		let mut found = [None, None, None, None];

		let Some(artifacts) = ticket.path.iter().next().map(Path::new) else {
			return Err(Error::NoTicket {
				id: ticket.id.clone(),
			});
		};

		let targets = Parser::new(text).filter_map(|event| match event {
			Event::Start(Tag::Link {
				dest_url,
				..
			}) => Some(dest_url.into_string()),
			_ => None,
		});

		for target in targets {
			let position = KINDS
				.iter()
				.position(|kind| target.ends_with(kind.suffix))
				.ok_or_else(|| Error::Kind {
					target: target.clone(),
				})?;
			let kind = KINDS[position];
			let home = kind.home.map(Path::new).unwrap_or(artifacts);
			let link = Link::locate(folder, &target, kind)?;

			if !link.path.starts_with(home) {
				return Err(Error::Outside {
					target,
					kind: kind.name,
					folder: home.display().to_string(),
				});
			}

			within(root, &link.path)?;

			if found[position].replace(link).is_some() {
				return Err(Error::Twice {
					kind: kind.name,
				});
			}
		}

		let [prd, design, spec, guardrail] = found;
		let missing = |kind: Kind| Error::Missing {
			kind: kind.name,
		};

		Ok(Self {
			prd: prd.ok_or_else(|| missing(PRD))?,
			design: design.ok_or_else(|| missing(DESIGN))?,
			spec: spec.ok_or_else(|| missing(SPEC))?,
			guardrail: guardrail.ok_or_else(|| missing(GUARDRAIL))?,
		})
	}
}
