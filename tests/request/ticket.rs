//! A ticket is found from its bare id and read by its header

use std::path::{Path, PathBuf};

use nsx::files::root::Root;
use nsx::request::{error::Error, ticket::Ticket};
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::request::project::project;
use crate::simulation::fixture::fixture;

const ID: &str = "0031-TICKET-book-a-demo-runtime";

const TEXT: &str = "# Book a demo\n\n**Status:** ready-for-agent\n";

#[test]
fn the_fixtures_ticket_resolves() {
	let found = Ticket::resolve(&fixture(), ID).unwrap();

	assert_eq!(
		found,
		Path::new(".agents/0031-TICKET-book-a-demo-runtime.ticket.md")
	);
}

#[rstest]
#[case::in_a_folder_of_agents(".agents/tickets/0007-TICKET-notes.ticket.md")]
#[case::in_claude(".claude/0007-TICKET-notes.ticket.md")]
#[case::deep_in_claude(".claude/work/tickets/0007-TICKET-notes.ticket.md")]
fn a_ticket_resolves_wherever_it_sits_in_the_artifact_folders(
	#[case] placed: &str,
) {
	let project = project(&[(placed, TEXT)]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let found = Ticket::resolve(&root, "0007-TICKET-notes").unwrap();

	assert_eq!(found, Path::new(placed));
}

#[rstest]
#[case::an_id_no_file_carries("0099-TICKET-nothing")]
#[case::an_id_that_climbs("../0007-TICKET-notes")]
#[case::an_id_with_a_folder("tickets/0007-TICKET-notes")]
#[case::part_of_an_id("0007")]
#[case::an_id_with_its_suffix("0007-TICKET-notes.ticket.md")]
fn an_id_that_names_no_ticket_is_a_stop(#[case] id: &str) {
	let project = project(&[
		(".agents/tickets/0007-TICKET-notes.ticket.md", TEXT),
		(".langdrift/0099-TICKET-nothing.ticket.md", TEXT),
		("docs/0099-TICKET-nothing.ticket.md", TEXT),
	]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Ticket::resolve(&root, id).unwrap_err();

	assert!(matches!(refusal, Error::NoTicket { .. }), "was {refusal:?}");
	assert!(refusal.to_string().contains(id), "said {refusal}");
}

#[test]
fn an_id_two_files_carry_is_a_stop() {
	let project = project(&[
		(".agents/0007-TICKET-notes.ticket.md", TEXT),
		(".claude/0007-TICKET-notes.ticket.md", TEXT),
	]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Ticket::resolve(&root, "0007-TICKET-notes").unwrap_err();

	assert!(matches!(refusal, Error::Tickets { .. }), "was {refusal:?}");
}

#[cfg(unix)]
#[test]
fn a_linked_folder_is_never_searched() {
	let elsewhere = project(&[("0007-TICKET-notes.ticket.md", TEXT)]);
	let project = project(&[(".agents/readme.guide.md", "")]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	std::os::unix::fs::symlink(
		elsewhere.path(),
		project.path().join(".agents/linked"),
	)
	.unwrap();

	let refusal = Ticket::resolve(&root, "0007-TICKET-notes").unwrap_err();

	assert!(matches!(refusal, Error::NoTicket { .. }), "was {refusal:?}");
}

#[test]
fn the_header_gives_the_number_the_slug_the_status_and_the_title() {
	let root = fixture();
	let found = Ticket::resolve(&root, ID).unwrap();
	let text = std::fs::read_to_string(root.path.join(&found)).unwrap();

	let ticket = Ticket::parse(ID, &found, &text).unwrap();

	assert_eq!(
		ticket,
		Ticket {
			id: String::from(ID),
			number: Some(String::from("0031")),
			slug: String::from("book-a-demo-runtime"),
			status: String::from("ready-for-agent"),
			title: String::from(
				"Run the book-a-demo request use case as a microruntime"
			),
			path: found,
		}
	);
}

#[rstest]
#[case::no_number("notes", None, "notes")]
#[case::a_number_and_no_marker("0007-notes", Some("0007"), "notes")]
#[case::a_marker_and_no_number("TICKET-notes", None, "notes")]
fn an_id_states_its_number_or_leaves_it_unknown(
	#[case] id: &str,
	#[case] number: Option<&str>,
	#[case] slug: &str,
) {
	let ticket =
		Ticket::parse(id, Path::new(".agents/x.ticket.md"), TEXT).unwrap();

	assert_eq!(ticket.number.as_deref(), number);
	assert_eq!(ticket.slug, slug);
}

#[test]
fn the_title_is_the_words_of_the_first_heading() {
	let text =
		"Notes first.\n\n## The `nsx` **plan**\n\n# Later\n\n**Status:** draft\n";

	let ticket =
		Ticket::parse("notes", Path::new(".agents/x.ticket.md"), text).unwrap();

	assert_eq!(ticket.title, "The nsx plan");
	assert_eq!(ticket.status, "draft");
}

#[rstest]
#[case::no_heading("**Status:** ready-for-agent\n")]
#[case::an_empty_heading("#\n\n**Status:** ready-for-agent\n")]
fn a_ticket_without_a_title_is_a_stop(#[case] text: &str) {
	let refusal =
		Ticket::parse("notes", Path::new(".agents/x.ticket.md"), text).unwrap_err();

	assert!(matches!(refusal, Error::NoTitle { .. }), "was {refusal:?}");
}

#[rstest]
#[case::no_status_line("# Notes\n")]
#[case::an_empty_status("# Notes\n\n**Status:**\n")]
#[case::a_status_in_a_sentence("# Notes\n\nIts **Status:** is ready.\n")]
fn a_ticket_without_a_status_is_a_stop(#[case] text: &str) {
	let refusal =
		Ticket::parse("notes", Path::new(".agents/x.ticket.md"), text).unwrap_err();

	assert!(matches!(refusal, Error::NoStatus { .. }), "was {refusal:?}");
}

#[test]
fn the_tickets_line_names_it_by_its_id() {
	let ticket =
		Ticket::parse(ID, Path::new(".agents/x.ticket.md"), TEXT).unwrap();

	let line = ticket.line();

	assert_eq!(line.id, "readTicket");
	assert_eq!(line.kind, Kind::Output);
	assert_eq!(
		line.render(),
		"console › tool call: read 0031-TICKET-book-a-demo-runtime"
	);
}

#[test]
fn the_intent_is_the_title_and_the_body_without_status_or_links() {
	let root = fixture();
	let found = Ticket::resolve(&root, ID).unwrap();
	let text = std::fs::read_to_string(root.path.join(&found)).unwrap();
	let ticket = Ticket::parse(ID, &found, &text).unwrap();

	assert_eq!(
		ticket.intent(&text),
		"Run the book-a-demo request use case as a microruntime\nThe book-a-demo request runs as one bounded microruntime: the sign-up API, the notify API, the broker, the cache and the sign-up web app."
	);
}

#[rstest]
#[case::nothing_after_the_number("0031-")]
#[case::nothing_after_the_marker("0031-TICKET-")]
#[case::capitals_and_underscores("0031-TICKET-Book_A_Demo")]
#[case::five_digits_and_a_marker("00071-TICKET-notes")]
#[case::a_folder_of_its_own("0031-TICKET-..")]
#[case::a_dash_at_the_end("0031-TICKET-notes-")]
fn an_id_that_leaves_no_name_for_a_runtime_is_a_stop(#[case] id: &str) {
	let refusal =
		Ticket::parse(id, Path::new(".agents/x.ticket.md"), TEXT).unwrap_err();

	assert!(matches!(refusal, Error::Slug { .. }), "was {refusal:?}");
	assert!(refusal.to_string().contains(id), "said {refusal}");
}

#[rstest]
#[case::a_title_underlined(
	"Book a demo\n===========\n\n**Status:** ready\n\nA visitor asks for a demo.\n\n## Links\n\n- [prd](./a.prd.md)\n"
)]
#[case::a_links_heading_closed_with_hashes(
	"# Book a demo\n\n**Status:** ready\n\nA visitor asks for a demo.\n\n## Links ##\n\n- [prd](./a.prd.md)\n"
)]
#[case::links_before_the_body(
	"# Book a demo\n\n**Status:** ready\n\n## Links\n\n- [prd](./a.prd.md)\n\n### More links\n\n- [dd](./a.dd.md)\n\n## Why\n\nA visitor asks for a demo.\n"
)]
fn the_intent_never_holds_the_status_or_the_links(#[case] written: &str) {
	let ticket =
		Ticket::parse("notes", Path::new(".agents/x.ticket.md"), written).unwrap();

	let intent = ticket.intent(written);

	assert!(intent.starts_with("Book a demo\n"), "was {intent:?}");
	assert!(
		intent.contains("A visitor asks for a demo."),
		"was {intent:?}"
	);
	assert!(!intent.contains(".md"), "was {intent:?}");
	assert!(!intent.contains("Status"), "was {intent:?}");
}
