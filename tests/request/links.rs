//! A ticket links one PRD, one design doc, one spec and one guardrail

use std::path::{Path, PathBuf};

use nsx::files::root::Root;
use nsx::request::{error::Error, links::Links, ticket::Ticket};
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::request::{linked::linked, project::project};
use crate::simulation::fixture::fixture;

const ID: &str = "0031-TICKET-book-a-demo-runtime";

const FOUR: &str = "- [prd](./0002-PRD-book-a-demo.prd.md)\n- [dd](./0005-DD-runtime.dd.md)\n- [spec](./0011-SPEC-runtime.spec.md)\n- [gr](/.langdrift/harness/guardrails/0001-finops.gr.md)\n";

#[test]
fn the_fixtures_ticket_links_its_four_documents() {
	let root = fixture();
	let found = Ticket::resolve(&root, ID).unwrap();
	let text = std::fs::read_to_string(root.path.join(&found)).unwrap();
	let ticket = Ticket::parse(ID, &found, &text).unwrap();

	let links = Links::parse(&root, &ticket, &text).unwrap();

	assert_eq!(
		[
			(links.prd.short.as_str(), links.prd.name.as_str()),
			(links.design.short.as_str(), links.design.name.as_str()),
			(links.spec.short.as_str(), links.spec.name.as_str()),
			(
				links.guardrail.short.as_str(),
				links.guardrail.name.as_str()
			),
		],
		[
			("0002-PRD", "0002-PRD-book-a-demo.prd.md"),
			("0005-DD", "0005-DD-book-a-demo-runtime.dd.md"),
			("0011-SPEC", "0011-SPEC-book-a-demo-runtime.spec.md"),
			("0001-finops", "0001-finops.gr.md"),
		]
	);
	assert_eq!(
		links.prd.path,
		Path::new(".agents/0002-PRD-book-a-demo.prd.md")
	);
	assert_eq!(
		links.guardrail.path,
		Path::new(".langdrift/harness/guardrails/0001-finops.gr.md")
	);
}

#[test]
fn the_links_line_states_the_status_and_names_the_four_by_short_id() {
	let root = fixture();
	let found = Ticket::resolve(&root, ID).unwrap();
	let text = std::fs::read_to_string(root.path.join(&found)).unwrap();
	let ticket = Ticket::parse(ID, &found, &text).unwrap();
	let links = Links::parse(&root, &ticket, &text).unwrap();

	let line = links.line(&ticket.status);

	assert_eq!(line.id, "links");
	assert_eq!(line.kind, Kind::Output);
	assert_eq!(
		line.render(),
		"console › ready-for-agent · links 0002-PRD · 0005-DD · 0011-SPEC · guardrail 0001-finops"
	);
}

#[test]
fn the_order_the_ticket_wrote_them_in_does_not_count() {
	let project = project(&[]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let body = "See [gr](/.langdrift/harness/guardrails/0001-finops.gr.md), then [spec](../specs/0011-SPEC-runtime.spec.md), [dd](../0005-DD-runtime.dd.md) and [prd](./notes.prd.md).\n";

	let links = linked(&root, ".agents/tickets/x.ticket.md", body).unwrap();

	assert_eq!(links.prd.short, "notes");
	assert_eq!(links.prd.path, Path::new(".agents/tickets/notes.prd.md"));
	assert_eq!(
		links.design.path,
		Path::new(".agents/0005-DD-runtime.dd.md")
	);
	assert_eq!(
		links.spec.path,
		Path::new(".agents/specs/0011-SPEC-runtime.spec.md")
	);
}

#[rstest]
#[case::a_plain_document("- [notes](./notes.md)\n")]
#[case::a_page("- [issue](https://example.com/issues/31)\n")]
#[case::another_ticket("- [next](./0032-TICKET-next.ticket.md)\n")]
fn a_link_of_another_kind_is_a_stop(#[case] extra: &str) {
	let project = project(&[]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let body = format!("{FOUR}{extra}");

	let refusal = linked(&root, ".agents/x.ticket.md", &body).unwrap_err();

	assert!(matches!(refusal, Error::Kind { .. }), "was {refusal:?}");
}

#[rstest]
#[case::no_prd("- [prd](./0002-PRD-book-a-demo.prd.md)\n", "PRD")]
#[case::no_design_doc("- [dd](./0005-DD-runtime.dd.md)\n", "design doc")]
#[case::no_spec("- [spec](./0011-SPEC-runtime.spec.md)\n", "spec")]
#[case::no_guardrail(
	"- [gr](/.langdrift/harness/guardrails/0001-finops.gr.md)\n",
	"guardrail"
)]
fn a_kind_the_ticket_does_not_link_is_a_stop(
	#[case] left_out: &str,
	#[case] kind: &str,
) {
	let project = project(&[]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let body = FOUR.replace(left_out, "");

	let refusal = linked(&root, ".agents/x.ticket.md", &body).unwrap_err();

	assert!(matches!(refusal, Error::Missing { .. }), "was {refusal:?}");
	assert!(refusal.to_string().contains(kind), "said {refusal}");
}

#[test]
fn a_kind_linked_twice_is_a_stop() {
	let project = project(&[]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let body = format!("{FOUR}- [other](./0003-PRD-other.prd.md)\n");

	let refusal = linked(&root, ".agents/x.ticket.md", &body).unwrap_err();

	assert!(
		matches!(
			refusal,
			Error::Twice {
				kind: "PRD"
			}
		),
		"was {refusal:?}"
	);
}

#[rstest]
#[case::a_prd_beside_the_code(
	"./0002-PRD-book-a-demo.prd.md",
	"/docs/0002-PRD-book-a-demo.prd.md"
)]
#[case::a_spec_in_the_other_artifact_folder(
	"./0011-SPEC-runtime.spec.md",
	"/.claude/0011-SPEC-runtime.spec.md"
)]
#[case::a_design_doc_one_folder_up(
	"./0005-DD-runtime.dd.md",
	"../0005-DD-runtime.dd.md"
)]
#[case::a_guardrail_among_the_documents(
	"/.langdrift/harness/guardrails/0001-finops.gr.md",
	"./0001-finops.gr.md"
)]
#[case::a_guardrail_elsewhere_in_langdrift(
	"/.langdrift/harness/guardrails/0001-finops.gr.md",
	"/.langdrift/skills/0001-finops.gr.md"
)]
fn a_link_outside_its_folder_is_a_stop(
	#[case] kept: &str,
	#[case] moved: &str,
) {
	let project = project(&[]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let body = FOUR.replace(kept, moved);

	let refusal = linked(&root, ".agents/x.ticket.md", &body).unwrap_err();

	assert!(matches!(refusal, Error::Outside { .. }), "was {refusal:?}");
}

#[test]
fn a_link_that_leaves_the_project_is_a_stop() {
	let project = project(&[]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let body = FOUR.replace(
		"./0002-PRD-book-a-demo.prd.md",
		"../../elsewhere/0002-PRD-book-a-demo.prd.md",
	);

	let refusal = linked(&root, ".agents/x.ticket.md", &body).unwrap_err();

	assert!(matches!(refusal, Error::Climbs { .. }), "was {refusal:?}");
}

#[cfg(unix)]
#[test]
fn a_link_that_is_a_symbolic_link_is_a_stop() {
	let elsewhere = project(&[("secret.prd.md", "# Secret\n")]);
	let project = project(&[(".agents/x.ticket.md", "")]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	std::os::unix::fs::symlink(
		elsewhere.path().join("secret.prd.md"),
		project.path().join(".agents/0002-PRD-book-a-demo.prd.md"),
	)
	.unwrap();

	let refusal = linked(&root, ".agents/x.ticket.md", FOUR).unwrap_err();

	assert!(matches!(refusal, Error::File(_)), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("symbolic link"),
		"said {refusal}"
	);
}
