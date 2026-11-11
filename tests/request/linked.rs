//! The links of a ticket written for a test: a header, then the body given

use std::path::Path;

use nsx::files::root::Root;
use nsx::request::{error::Error, links::Links, ticket::Ticket};

const ID: &str = "0031-TICKET-book-a-demo-runtime";

const HEADER: &str = "# Book a demo\n\n**Status:** ready-for-agent\n\n";

pub fn linked(root: &Root, placed: &str, body: &str) -> Result<Links, Error> {
	let text = format!("{HEADER}{body}");
	let ticket = Ticket::parse(ID, Path::new(placed), &text).unwrap();

	Links::parse(root, &ticket, &text)
}
