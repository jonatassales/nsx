//! Reading the ticket's header
//!
//! One function: it takes the ticket's id, its project-relative path and the
//! text of its file, and gives back the ticket: the number and the slug from
//! the id, the title from the document's first heading and the status from its
//! `**Status:**` line.
//!
//! ## Rules
//!
//! - The number is the id's first four digits. The slug is what follows
//!   `<number>-TICKET-`.
//! - An id that opens with no four digits and a dash has no number, and its
//!   slug is the id.
//! - The slug is the runtime's id and its folder's name, so it must be a
//!   lowercase DNS label. An id that leaves any other slug stops the plan.
//! - The header is read before the plan's first line and handed to the plan's
//!   card data first.
//! - A ticket without a title or a status line stops the plan (the stop rule).
//!   There is no "no status" fallback.

use std::path::Path;

use pulldown_cmark::{Event, Parser, Tag, TagEnd};

use crate::contract::label::label;
use crate::request::{error::Error, ticket::Ticket};

const NUMBER_DIGITS: usize = 4;

impl Ticket {
	pub fn parse(id: &str, path: &Path, text: &str) -> Result<Self, Error> {
		let numbered = id.len() > NUMBER_DIGITS
			&& id.is_char_boundary(NUMBER_DIGITS)
			&& id[..NUMBER_DIGITS]
				.chars()
				.all(|digit| digit.is_ascii_digit())
			&& id[NUMBER_DIGITS..].starts_with('-');
		let (number, unnumbered) = if numbered {
			(Some(&id[..NUMBER_DIGITS]), &id[NUMBER_DIGITS + 1..])
		} else {
			(None, id)
		};
		let slug = unnumbered.strip_prefix("TICKET-").unwrap_or(unnumbered);

		// The title is the words of the first heading, whatever its level.
		let title = Parser::new(text)
			.skip_while(|event| !matches!(event, Event::Start(Tag::Heading { .. })))
			.take_while(|event| !matches!(event, Event::End(TagEnd::Heading(_))))
			.filter_map(|event| match event {
				Event::Text(words) | Event::Code(words) => Some(words.into_string()),
				_ => None,
			})
			.collect::<String>();
		let status = text
			.lines()
			.find_map(|line| line.strip_prefix("**Status:**"))
			.map(str::trim);

		if title.trim().is_empty() {
			return Err(Error::NoTitle {
				path: path.to_path_buf(),
			});
		}

		let Some(status) = status.filter(|stated| !stated.is_empty()) else {
			return Err(Error::NoStatus {
				path: path.to_path_buf(),
			});
		};

		// The slug names the runtime and its folder.
		if !label(slug) {
			return Err(Error::Slug {
				id: String::from(id),
			});
		}

		Ok(Self {
			id: String::from(id),
			number: number.map(String::from),
			slug: String::from(slug),
			status: String::from(status),
			title: String::from(title.trim()),
			path: path.to_path_buf(),
		})
	}
}
