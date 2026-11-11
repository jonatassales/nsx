//! Each linked document gives the facts of its kind, by fixed rule

use std::path::PathBuf;

use nsx::files::root::Root;
use nsx::money::{budget::Budget, cents::Cents};
use nsx::request::{
	design::Design, error::Error, links::Links, named::Named, prd::Prd,
	reading::Reading, spec::Spec, ticket::Ticket,
};
use rstest::rstest;

use crate::request::{placed::placed, project::project};
use crate::simulation::fixture::fixture;

const ID: &str = "0031-TICKET-book-a-demo-runtime";

#[test]
fn the_fixtures_three_documents_give_the_facts_the_goldens_print() {
	let root = fixture();
	let found = Ticket::resolve(&root, ID).unwrap();
	let text = std::fs::read_to_string(root.path.join(&found)).unwrap();
	let ticket = Ticket::parse(ID, &found, &text).unwrap();
	let links = Links::parse(&root, &ticket, &text).unwrap();

	let prd = Prd::read(&root, &links.prd).unwrap();
	let design = Design::read(&root, &links.design).unwrap();
	let spec = Spec::read(&root, &links.spec).unwrap();
	let named = spec
		.components
		.iter()
		.map(|named| (named.path.as_str(), named.port))
		.collect::<Vec<_>>();

	assert_eq!(
		prd.line().render(),
		"console › tool call: read 0002-PRD · use case: book-a-demo request"
	);
	assert_eq!(
		design.line().render(),
		"console › tool call: read 0005-DD · envelope cpuMillis 2000 · memoryMiB 3072 · set by the team"
	);
	assert_eq!(
		spec.line().render(),
		"console › tool call: read 0011-SPEC · 5 components named · 3 acceptance criteria"
	);
	assert_eq!(
		(design.cpu_millis, design.memory_mib, design.budget),
		(2000, 3072, Cents(2500))
	);
	assert_eq!(
		named,
		[
			("services/signup-api", Some(8080)),
			("services/notify-api", None),
			("platform/nats", None),
			("platform/redis", None),
			("apps/signup", None),
		]
	);
	assert_eq!(spec.criteria, 3);
	assert_eq!(
		Budget::of(&design).line().render(),
		"budget     $25.00 per run for model calls · set by the team in the design doc"
	);
	assert_eq!(
		Reading::count(&links).line().render(),
		"✓ read       4 documents · nothing outside the ticket's links"
	);
}

#[rstest]
#[case::with_cents("$25.00", 2500)]
#[case::without_cents("$25", 2500)]
#[case::thousands("$1,250.50", 125_050)]
#[case::millions("$1,000,000", 100_000_000)]
#[case::ungrouped("$1250.50", 125_050)]
#[case::under_a_dollar("$0.05", 5)]
#[case::nothing("$0.00", 0)]
fn a_budget_reads_as_whole_cents(#[case] written: &str, #[case] cents: i64) {
	let design = format!(
		"# Notes\n\n**cpuMillis:** 2000\n\n**memoryMiB:** 3072\n\n**Budget per run:** {written}\n"
	);
	let project = project(&[(".agents/notes.dd.md", &design)]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let document = Design::read(&root, &placed("notes.dd.md")).unwrap();

	assert_eq!(document.budget, Cents(cents));
}

#[rstest]
#[case::no_dollar_sign("**Budget per run:** 25.00", "Budget per run")]
#[case::one_decimal("**Budget per run:** $25.5", "Budget per run")]
#[case::a_decimal_comma("**Budget per run:** $25,00", "Budget per run")]
#[case::a_group_of_two("**Budget per run:** $1,25.00", "Budget per run")]
#[case::a_leading_group_of_four(
	"**Budget per run:** $1250,000",
	"Budget per run"
)]
#[case::below_zero("**Budget per run:** $-5.00", "Budget per run")]
#[case::words("**Budget per run:** twenty", "Budget per run")]
#[case::a_cpu_in_words("**cpuMillis:** two cores", "cpuMillis")]
#[case::no_cpu_at_all("**cpuMillis:** 0", "cpuMillis")]
#[case::a_memory_with_its_unit("**memoryMiB:** 3072 MiB", "memoryMiB")]
#[case::a_memory_below_zero("**memoryMiB:** -5", "memoryMiB")]
fn a_malformed_field_is_a_stop(#[case] line: &str, #[case] field: &str) {
	let mut lines = vec![
		"**cpuMillis:** 2000",
		"**memoryMiB:** 3072",
		"**Budget per run:** $25.00",
	];

	lines.retain(|kept| !kept.starts_with(&format!("**{field}:**")));
	lines.push(line);

	let design = format!("# Notes\n\n{}\n", lines.join("\n\n"));
	let project = project(&[(".agents/notes.dd.md", &design)]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Design::read(&root, &placed("notes.dd.md")).unwrap_err();

	assert!(
		matches!(refusal, Error::Malformed { .. }),
		"was {refusal:?}"
	);
	assert!(refusal.to_string().contains(field), "said {refusal}");
}

#[rstest]
#[case::no_budget(
	"**cpuMillis:** 2000\n\n**memoryMiB:** 3072\n",
	"Budget per run"
)]
#[case::no_memory(
	"**cpuMillis:** 2000\n\n**Budget per run:** $25.00\n",
	"memoryMiB"
)]
#[case::no_cpu(
	"**memoryMiB:** 3072\n\n**Budget per run:** $25.00\n",
	"cpuMillis"
)]
fn a_design_doc_without_a_field_is_a_stop(
	#[case] written: &str,
	#[case] field: &str,
) {
	let project = project(&[(".agents/notes.dd.md", written)]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Design::read(&root, &placed("notes.dd.md")).unwrap_err();

	assert!(matches!(refusal, Error::Field { .. }), "was {refusal:?}");
	assert!(refusal.to_string().contains(field), "said {refusal}");
}

#[rstest]
#[case::no_use_case("# Notes\n")]
#[case::an_empty_use_case("**Use case:**\n")]
#[case::a_use_case_inside_a_sentence("The **Use case:** is a demo.\n")]
fn a_prd_without_its_use_case_is_a_stop(#[case] written: &str) {
	let project = project(&[(".agents/notes.prd.md", written)]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Prd::read(&root, &placed("notes.prd.md")).unwrap_err();

	assert!(matches!(refusal, Error::Field { .. }), "was {refusal:?}");
	assert!(refusal.to_string().contains("Use case"), "said {refusal}");
}

#[rstest]
#[case::no_components(
	"## Acceptance criteria\n\n- [ ] It works.\n",
	"Components"
)]
#[case::no_criteria(
	"## Components\n\n- `services/api`\n",
	"Acceptance criteria"
)]
#[case::a_section_with_no_item(
	"## Components\n\nNone yet.\n\n## Acceptance criteria\n\n- [ ] It works.\n",
	"Components"
)]
fn a_spec_without_a_list_is_a_stop(
	#[case] written: &str,
	#[case] section: &str,
) {
	let project = project(&[(".agents/notes.spec.md", written)]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Spec::read(&root, &placed("notes.spec.md")).unwrap_err();

	assert!(matches!(refusal, Error::Section { .. }), "was {refusal:?}");
	assert!(refusal.to_string().contains(section), "said {refusal}");
}

#[rstest]
#[case::no_backticks("- services/api")]
#[case::a_port_in_words("- `services/api` `:http`")]
#[case::a_port_of_zero("- `services/api` `:0`")]
#[case::a_port_past_the_last("- `services/api` :65536")]
fn a_component_the_spec_does_not_name_well_is_a_stop(#[case] item: &str) {
	let written = format!(
		"## Components\n\n{item}\n\n## Acceptance criteria\n\n- [ ] It works.\n"
	);
	let project = project(&[(".agents/notes.spec.md", &written)]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Spec::read(&root, &placed("notes.spec.md")).unwrap_err();

	assert!(
		matches!(refusal, Error::Component { .. }),
		"was {refusal:?}"
	);
}

#[test]
fn a_spec_names_a_port_beside_a_path_or_leaves_it_unknown() {
	let written = "## Components\n\n- `services/api` :8080\n- `services/worker` the worker\n- `apps/web` `:3000`\n\n## Later\n\n- `ignored/here`\n\n## Acceptance criteria\n\n- [ ] One.\n- [x] Two.\n";
	let project = project(&[(".agents/notes.spec.md", written)]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let spec = Spec::read(&root, &placed("notes.spec.md")).unwrap();

	assert_eq!(
		spec.components,
		[
			Named {
				path: String::from("services/api"),
				port: Some(8080),
			},
			Named {
				path: String::from("services/worker"),
				port: None,
			},
			Named {
				path: String::from("apps/web"),
				port: Some(3000),
			},
		]
	);
	assert_eq!(spec.criteria, 2);
}

#[test]
fn a_document_that_is_not_there_is_a_stop() {
	let project = project(&[]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Prd::read(&root, &placed("notes.prd.md")).unwrap_err();

	assert!(matches!(refusal, Error::File(_)), "was {refusal:?}");
}

#[test]
fn a_folder_where_a_document_should_be_is_a_stop() {
	let project = project(&[(".agents/notes.prd.md/inside.md", "# Notes\n")]);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Prd::read(&root, &placed("notes.prd.md")).unwrap_err();

	assert!(
		refusal.to_string().contains("not a regular file"),
		"said {refusal}"
	);
}
