//! The ticket's intent
//!
//! One function: it takes the text of the ticket's file and gives back the
//! text the plan embeds to recall past runs: the ticket's title and body.
//!
//! ## Rules
//!
//! - The body is what the ticket says after its title, without its
//!   `**Status:**` line and without its Links section: neither says what the
//!   ticket wants (owner, 2026-11-11).
//! - The title and the Links section are found as Markdown headings, however
//!   the ticket writes them. The section runs to the next heading that is not
//!   beneath it.
//! - The same text embeds a finished run's episode, so a ticket planned twice
//!   meets itself.

use pulldown_cmark::{Event, Parser, Tag, TagEnd};

use crate::request::ticket::Ticket;

impl Ticket {
	pub fn intent(&self, written: &str) -> String {
		// Every heading of the ticket: where it sits, its level and its words.
		let mut headings = Vec::new();
		let mut open = None;

		for (event, span) in Parser::new(written).into_offset_iter() {
			match (event, open.as_mut()) {
				(
					Event::Start(Tag::Heading {
						level,
						..
					}),
					_,
				) => open = Some((span, level, String::new())),
				(
					Event::Text(words) | Event::Code(words),
					Some((_span, _level, said)),
				) => said.push_str(&words),
				(Event::End(TagEnd::Heading(_)), _) => headings.extend(open.take()),
				_ => {}
			}
		}

		let Some((title, _level, _said)) = headings.first() else {
			return self.title.clone();
		};

		// The links section runs from its heading to the next heading that is
		// not beneath it.
		let links = headings
			.iter()
			.position(|(_span, _level, said)| said.trim() == "Links")
			.map(|at| {
				let (span, level, _said) = &headings[at];
				let end = headings[at + 1..]
					.iter()
					.find(|(_span, next, _said)| next <= level)
					.map_or(written.len(), |(next, _level, _said)| next.start);

				span.start..end
			});
		let (before, after) = match &links {
			Some(section) if section.start >= title.end => {
				(&written[title.end..section.start], &written[section.end..])
			}
			_ => (&written[title.end..], ""),
		};
		let body = before
			.lines()
			.chain(after.lines())
			.filter(|line| !line.starts_with("**Status:**"))
			.collect::<Vec<_>>()
			.join("\n");

		format!("{}\n{}", self.title, body.trim())
	}
}
