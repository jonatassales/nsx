//! The plan: `nsx agent plan 0031-TICKET-book-a-demo-runtime`

use std::cell::RefCell;
use std::path::PathBuf;

use assert_cmd::Command;

use nsx::command::{init::Init, plan::Plan};
use nsx::contract::{
	cards::Cards, decode::decode, fence::Fence, intent::Intent, schema::schema,
};
use nsx::session::title::Title;
use rstest::rstest;

use crate::simulation::{
	diff::diff,
	fake::{
		judge::judge, recorder::Recorder, refuser::Refuser, session::session,
		witness::Witness,
	},
	golden::golden,
	indexed::indexed,
	pending,
	printed::printed,
	project::project,
};

#[tokio::test]
async fn the_plan_plays_as_the_simulation_does() {
	let project = project();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};

	let judge = judge();

	Init.execute(&mut session).await.unwrap();
	indexed(project.path());

	session.judge = &judge;

	plan.execute(&mut session).await.unwrap();

	let differences = diff(&golden("plan"), &printed(&recorder), &pending::PLAN);
	let written = std::fs::read_to_string(
		project
			.path()
			.join(".langdrift/microruntimes/runtimes/book-a-demo-runtime/plan.json"),
	)
	.unwrap();
	let cards = decode::<Cards>(&written).unwrap();
	let analysis = cards.analysis.unwrap();
	let documents = analysis.documents.unwrap();
	let episodes = analysis.episodes.unwrap();
	let suggestion = analysis.suggestion.unwrap();
	let rows = episodes.candidates.unwrap();
	let planned = cards.plan.clone().unwrap();
	let components = planned.components.unwrap().rows.unwrap();
	let sizing = serde_json::to_value(planned.sizing.unwrap()).unwrap();
	let draft =
		std::fs::read_to_string(project.path().join(
			".langdrift/microruntimes/runtimes/book-a-demo-runtime/runtime.md",
		))
		.unwrap();
	let fence = Fence::parse(&draft).unwrap();
	let intent = decode::<Intent>(
		&std::fs::read_to_string(project.path().join(
			".langdrift/microruntimes/runtimes/book-a-demo-runtime/intent.json",
		))
		.unwrap(),
	)
	.unwrap();

	assert!(differences.is_empty(), "{}", differences.join("\n"));
	assert!(jsonschema::is_valid(
		&schema::<Cards>(),
		&serde_json::from_str(&written).unwrap()
	));
	assert_eq!(
		(documents.read_count, documents.documents_read),
		(Some(4), Some(4))
	);
	assert_eq!(documents.prd.unwrap().use_case, "book-a-demo request");
	assert_eq!(documents.design_doc.unwrap().budget_per_run_cents, 2500);
	assert_eq!(documents.spec.unwrap().components, 5);
	assert_eq!(
		documents.guardrail.unwrap().criteria[1]
			.statement
			.as_deref(),
		Some("a budget before a paid run")
	);
	assert_eq!(
		(episodes.kept, episodes.dropped, episodes.total),
		(Some(3), Some(3), Some(6))
	);
	assert_eq!(
		(episodes.cited, episodes.cited_measured),
		(Some(3), Some(0))
	);
	assert_eq!(rows[0].similarity, 0.81);
	assert_eq!(rows[3].verdict.as_deref(), Some("dropped"));
	assert_eq!(rows[3].reason.as_deref(), Some("docs only"));
	assert_eq!(suggestion.model.as_deref(), Some("claude-sonnet-5-5"));
	assert_eq!(
		sizing,
		serde_json::json!({
			"bound": {
				"cpuMillis": 2000,
				"memoryMiB": 3072,
				"source": ".agents/0005-DD-book-a-demo-runtime.dd.md",
			},
			"proposed": {
				"cpuMillis": 1450,
				"memoryMiB": 2304,
			},
			"headroom": {
				"cpuMillis": 550,
				"memoryMiB": 768,
			},
			"resource": {
				"kind": "Deployment",
				"count": 5,
				"replicas": 1,
			},
			"limitsRequested": [],
			"validation": {
				"schema": "https://langdrift.md/schemas/configuration/runtime.schema.json",
				"errors": 0,
			},
		})
	);
	assert_eq!(planned.price.unwrap().source, None);
	assert!(fence.validate().is_empty());
	assert_eq!(
		(fence.runtime_id.as_str(), fence.revision),
		("book-a-demo-runtime", 1)
	);
	assert_eq!(
		(fence.model.as_str(), fence.effort.as_str()),
		("claude-sonnet-5-5", "medium")
	);
	assert_eq!(
		fence
			.components
			.iter()
			.map(|component| component.image.as_str())
			.collect::<Vec<_>>(),
		[
			"book-a-demo-runtime/signup-api:r1",
			"book-a-demo-runtime/notify-api:r1",
			"book-a-demo-runtime/nats:r1",
			"book-a-demo-runtime/redis:r1",
			"book-a-demo-runtime/signup:r1",
		]
	);
	assert!(draft.contains("| Headroom | 550m | 768 MiB |"));
	assert_eq!(intent.status, "proposed");
	assert_eq!(
		(
			intent.model.as_str(),
			intent.effort.as_str(),
			intent.revision
		),
		("claude-sonnet-5-5", "medium", 1)
	);
	assert_eq!(planned.evidence.unwrap().revision, Some(1));
	assert_eq!(
		components
			.iter()
			.map(|row| (row.path.as_str(), row.port, row.cpu_millis, row.memory_mib))
			.collect::<Vec<_>>(),
		[
			("services/signup-api", Some(8080), Some(250), Some(256)),
			("services/notify-api", Some(8081), Some(300), Some(512)),
			("platform/nats", Some(4222), Some(200), Some(256)),
			("platform/redis", Some(6379), Some(200), Some(256)),
			("apps/signup", Some(3000), Some(500), Some(1024)),
		]
	);
	assert_eq!(suggestion.by.as_deref(), Some("recommendation"));
	assert_eq!(episodes.threshold, Some(0.55));
	assert_eq!(
		cards
			.ticket
			.unwrap()
			.links
			.unwrap()
			.iter()
			.map(|linked| (linked.kind.as_str(), linked.short_id.as_str()))
			.collect::<Vec<_>>(),
		[
			("prd", "0002-PRD"),
			("designDoc", "0005-DD"),
			("spec", "0011-SPEC"),
			("guardrail", "0001-finops"),
		]
	);
}

#[rstest]
#[case::the_tickets_links("/ticket/links", "links")]
#[case::the_prds_row("/analysis/documents/prd", "readPrd")]
#[case::the_design_docs_row("/analysis/documents/designDoc", "readDesignDoc")]
#[case::the_specs_row("/analysis/documents/spec", "readSpec")]
#[case::the_documents_read("/analysis/documents/documentsRead", "reads")]
#[case::the_guardrails_row("/analysis/documents/guardrail", "guardrail")]
#[case::the_criterias_statements(
	"/analysis/documents/guardrail/criteria/0/statement",
	"judgeGuardrail"
)]
#[case::the_candidates("/analysis/episodes/candidates", "candidates")]
#[case::the_episodes_kept("/analysis/episodes/kept", "drop")]
#[case::the_episodes_cited("/analysis/episodes/cited", "episodes")]
#[case::the_suggested_model("/analysis/suggestion/model", "suggest")]
#[case::the_suggestions_reasons("/analysis/suggestion/effortReason", "suggest")]
#[case::the_placeholder_rows("/plan/components/rows", "resolve")]
#[case::the_first_components_stack("/plan/components/rows/0/stack", "rust")]
#[case::the_last_components_share("/plan/components/rows/4/cpuMillis", "next")]
#[case::the_first_components_image(
	"/plan/components/rows/0/image",
	"crossCheck"
)]
#[case::the_ports_distinct("/plan/components/portsDistinct", "crossCheck")]
#[case::the_bound("/plan/sizing/bound", "bounds")]
#[case::the_proposed_totals("/plan/sizing/proposed", "rust")]
#[case::the_headroom("/plan/sizing/headroom", "size")]
#[case::the_suggested_resource("/plan/sizing/resource", "size")]
#[case::the_limits("/plan/sizing/limitsRequested", "limits")]
#[case::the_price("/plan/price", "price")]
#[case::the_validation("/plan/sizing/validation", "validate")]
#[case::the_authorization("/plan/authorization", "draft")]
#[case::the_evidence("/plan/evidence", "draft")]
#[tokio::test]
async fn a_field_is_absent_until_its_line_has_printed(
	#[case] field: &str,
	#[case] line: &str,
) {
	let project = project();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let judge = judge();
	let witness = Witness {
		file: project
			.path()
			.join(".langdrift/microruntimes/runtimes/book-a-demo-runtime/plan.json"),
		seen: RefCell::default(),
	};

	Init.execute(&mut session).await.unwrap();
	indexed(project.path());

	session.judge = &judge;
	session.sink = &witness;

	plan.execute(&mut session).await.unwrap();

	let seen = witness.seen.borrow();
	let at = seen.iter().position(|(id, _cards)| id == line).unwrap();

	assert!(seen[at].1.pointer(field).is_none(), "{field} at {line}");
	assert!(
		seen[at + 1].1.pointer(field).is_some(),
		"{field} after {line}"
	);
}

#[tokio::test]
async fn the_header_threshold_and_spend_table_are_there_before_the_first_line()
{
	let project = project();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let witness = Witness {
		file: project
			.path()
			.join(".langdrift/microruntimes/runtimes/book-a-demo-runtime/plan.json"),
		seen: RefCell::default(),
	};

	Init.execute(&mut session).await.unwrap();

	session.sink = &witness;

	let stop = plan.execute(&mut session).await.unwrap_err();
	let seen = witness.seen.borrow();
	let (first, cards) = &seen[0];

	assert!(stop.sentence.contains("no judge"), "said {stop:?}");
	assert_eq!(first, "scope");
	assert_eq!(cards["ticket"]["slug"], "book-a-demo-runtime");
	assert_eq!(cards["analysis"]["episodes"]["threshold"], 0.55);
	assert_eq!(cards["workflow"]["budgetCents"], 2500);
	assert_eq!(cards["workflow"]["marginPercent"], 20);
	assert_eq!(cards["workflow"]["spend"].as_array().unwrap().len(), 9);
	assert!(cards.pointer("/ticket/links").is_none());
}

#[tokio::test]
async fn a_second_plan_writes_the_next_revision_over_the_draft() {
	let project = project();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let judge = judge();
	let file = project
		.path()
		.join(".langdrift/microruntimes/runtimes/book-a-demo-runtime/runtime.md");

	Init.execute(&mut session).await.unwrap();
	indexed(project.path());

	session.judge = &judge;

	plan.execute(&mut session).await.unwrap();
	plan.execute(&mut session).await.unwrap();

	let draft = std::fs::read_to_string(&file).unwrap();

	let intent = decode::<Intent>(
		&std::fs::read_to_string(file.with_file_name("intent.json")).unwrap(),
	)
	.unwrap();

	assert_eq!(Fence::parse(&draft).unwrap().revision, 2);
	assert_eq!((intent.revision, intent.status.as_str()), (2, "proposed"));
}

#[rstest]
#[case::crossed_at_the_last_component("1000")]
#[case::crossed_at_the_first_component("100")]
#[tokio::test]
async fn a_plan_over_the_envelope_stops_before_size_with_the_whole_sum(
	#[case] cpu_millis: &str,
) {
	let project = project();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let judge = judge();
	let folder = project
		.path()
		.join(".langdrift/microruntimes/runtimes/book-a-demo-runtime");
	let design = project
		.path()
		.join(".agents/0005-DD-book-a-demo-runtime.dd.md");
	let written = std::fs::read_to_string(&design).unwrap();

	std::fs::write(&design, written.replace("2000", cpu_millis)).unwrap();
	Init.execute(&mut session).await.unwrap();
	indexed(project.path());

	session.judge = &judge;

	let stop = plan.execute(&mut session).await.unwrap_err();
	let lines = recorder.lines.borrow();
	let cards = decode::<Cards>(
		&std::fs::read_to_string(folder.join("plan.json")).unwrap(),
	)
	.unwrap();
	let planned = cards.plan.unwrap();
	let sizing = planned.sizing.unwrap();
	let proposed = sizing.proposed.unwrap();

	assert_eq!(
		stop.sentence,
		format!(
			"The components need 1450m and 2304 MiB, over the bound of {cpu_millis}m and 3072 MiB the team set in .agents/0005-DD-book-a-demo-runtime.dd.md."
		)
	);
	assert_eq!(lines.last().unwrap().id, "crossCheck");
	assert_eq!((proposed.cpu_millis, proposed.memory_mib), (1450, 2304));
	assert!(sizing.headroom.is_none());
	assert!(
		planned
			.components
			.unwrap()
			.rows
			.unwrap()
			.iter()
			.all(|row| row.image.is_some())
	);
	assert!(!folder.join("runtime.md").exists());
	assert!(!folder.join("intent.json").exists());
}

#[test]
fn a_document_that_carries_an_escape_stops_the_plan_at_its_line() {
	let project = project();
	let nsx = || {
		let mut command = Command::cargo_bin("nsx").unwrap();

		command.current_dir(project.path());

		command
	};

	std::fs::write(
		project.path().join(".agents/0002-PRD-book-a-demo.prd.md"),
		"# Book a demo\n\n**Use case:** a demo\u{1b}[2Kconsole › forged\n",
	)
	.unwrap();

	nsx().arg("init").assert().success();

	let output = nsx()
		.args(["agent", "plan", "0031-TICKET-book-a-demo-runtime"])
		.output()
		.unwrap();
	let printed = String::from_utf8(output.stdout).unwrap();

	assert_eq!(output.status.code(), Some(1));
	assert_eq!(printed.lines().count(), 4, "printed {printed:?}");
	assert!(!printed.contains('\u{1b}'));
	assert_eq!(
		String::from_utf8(output.stderr).unwrap(),
		"nsx will not print the line readPrd: it carries a control character.\n"
	);
}

#[tokio::test]
async fn the_plan_names_the_project_in_the_window_title() {
	let project = project();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let folder = project.path().file_name().unwrap().to_string_lossy();

	let judge = judge();

	Init.execute(&mut session).await.unwrap();
	indexed(project.path());

	session.directory = project.path().join("services");
	session.judge = &judge;

	plan.execute(&mut session).await.unwrap();

	assert_eq!(
		*recorder.title.borrow(),
		Some(Title {
			project: folder.into_owned(),
		})
	);
}

#[tokio::test]
async fn a_plan_outside_any_project_stops_before_its_first_line() {
	let elsewhere = tempfile::tempdir().unwrap();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(elsewhere.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};

	let stop = plan.execute(&mut session).await.unwrap_err();

	assert!(stop.sentence.contains("nsx init"), "said {stop:?}");
	assert!(recorder.lines.borrow().is_empty());
}

#[tokio::test]
async fn the_tickets_header_is_in_plan_json_before_the_first_line() {
	let project = project();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let file = project
		.path()
		.join(".langdrift/microruntimes/runtimes/book-a-demo-runtime/plan.json");

	Init.execute(&mut session).await.unwrap();

	session.sink = &Refuser;

	let stop = plan.execute(&mut session).await.unwrap_err();
	let cards = decode::<Cards>(&std::fs::read_to_string(file).unwrap()).unwrap();
	let header = cards.ticket.unwrap();

	assert_eq!(stop.sentence, "the line scope was refused.");
	assert_eq!(header.id, "0031-TICKET-book-a-demo-runtime");
	assert_eq!(header.number.as_deref(), Some("0031"));
	assert_eq!(header.slug, "book-a-demo-runtime");
	assert_eq!(header.status.as_deref(), Some("ready-for-agent"));
	assert_eq!(
		header.title.as_deref(),
		Some("Run the book-a-demo request use case as a microruntime")
	);
}

#[test]
fn typed_in_a_project_with_no_judge_the_plan_stops_at_its_first_question() {
	let project = project();
	let nsx = || {
		let mut command = Command::cargo_bin("nsx").unwrap();

		command.current_dir(project.path());

		command
	};

	nsx().arg("init").assert().success();

	let output = nsx()
		.args(["agent", "plan", "0031-TICKET-book-a-demo-runtime"])
		.output()
		.unwrap();
	let printed = String::from_utf8(output.stdout).unwrap();
	let golden = golden("plan");
	let through_the_guardrail = golden
		.iter()
		.take_while(|line| line.id != "judgeGuardrail")
		.map(|line| format!("{}\n", line.text))
		.collect::<String>();

	assert_eq!(output.status.code(), Some(1));
	assert_eq!(printed, through_the_guardrail);
	assert_eq!(
		String::from_utf8(output.stderr).unwrap(),
		"nsx has no judge to ask yet: the plan stops here until the console's judge is connected.\n"
	);
}
