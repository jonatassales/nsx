//! The fence reads as version 3 and names every rule it breaks
//!
//! Ported from Core's Go tests, in its package `configuration` (removed with
//! ticket 0434): the same fence breaks the same rules, field by field.

use nsx::contract::{component::Component, error::Error, fence::Fence};
use rstest::rstest;

use crate::contract::{markdown::markdown, seventeen::seventeen};

#[test]
fn the_simulations_fence_reads_as_version_3() {
	let fence = Fence::parse(&markdown()).unwrap();

	assert_eq!(fence.version, 3);
	assert_eq!(fence.runtime_id, "book-a-demo-runtime");
	assert_eq!(fence.revision, 1);
	assert_eq!((fence.cpu_millis, fence.memory_mib), (2000, 3072));
	assert_eq!(fence.components.len(), 5);
	assert_eq!(
		fence.components[0],
		Component {
			path: String::from("services/signup-api"),
			stack: String::from("Rust"),
			role: String::from("api"),
			port: 8080,
			image: String::from("book-a-demo-runtime/signup-api:r1"),
			cpu_millis: 250,
			memory_mib: 256,
			replicas: 1,
		}
	);
}

#[test]
fn a_field_version_3_does_not_define_is_refused() {
	let markdown = markdown().replacen(
		"\"version\": 3,",
		"\"version\": 3, \"runtimeUid\": \"uid\",",
		1,
	);

	let refusal = Fence::parse(&markdown).unwrap_err();

	assert!(matches!(refusal, Error::Fence { .. }), "was {refusal:?}");
	assert!(refusal.to_string().contains("re-plan the runtime"));
}

#[test]
fn an_older_fence_is_refused_with_re_plan() {
	let markdown = markdown().replacen("\"version\": 3,", "\"version\": 2,", 1);

	let refusal = Fence::parse(&markdown).unwrap_err();

	assert!(matches!(refusal, Error::Version), "was {refusal:?}");
	assert!(refusal.to_string().contains("re-plan the runtime"));
}

#[test]
fn two_fences_are_refused() {
	let markdown = markdown().repeat(2);

	let refusal = Fence::parse(&markdown).unwrap_err();

	assert!(matches!(refusal, Error::Blocks), "was {refusal:?}");
}

#[test]
fn an_unclosed_fence_is_refused() {
	let markdown = markdown().replace("\n```\n", "\n");

	let refusal = Fence::parse(&markdown).unwrap_err();

	assert!(matches!(refusal, Error::Unclosed), "was {refusal:?}");
}

#[test]
fn a_runtime_md_over_1_mib_is_refused() {
	let markdown = format!("{}{}", " ".repeat(1 << 20), markdown());

	let refusal = Fence::parse(&markdown).unwrap_err();

	assert!(matches!(refusal, Error::TooLarge), "was {refusal:?}");
}

#[test]
fn the_simulations_fence_breaks_no_rule() {
	let fence = Fence::parse(&markdown()).unwrap();

	assert_eq!(fence.validate(), Vec::new());
}

#[rstest]
#[case::another_version(|fence: &mut Fence| fence.version = 2, "version")]
#[case::an_identifier_that_is_not_a_dns_label(
	|fence: &mut Fence| fence.runtime_id = String::from("Book_A_Demo"),
	"runtimeId"
)]
#[case::revision_0(|fence: &mut Fence| fence.revision = 0, "revision")]
#[case::a_cpu_bound_over_the_limit(
	|fence: &mut Fence| fence.cpu_millis = 64_001,
	"cpuMillis"
)]
#[case::a_memory_bound_over_the_limit(
	|fence: &mut Fence| fence.memory_mib = 1_048_577,
	"memoryMiB"
)]
#[case::no_model(|fence: &mut Fence| fence.model.clear(), "model")]
#[case::an_effort_outside_the_three(
	|fence: &mut Fence| fence.effort = String::from("max"),
	"effort"
)]
#[case::no_component(|fence: &mut Fence| fence.components.clear(), "components")]
#[case::seventeen_components(
	|fence: &mut Fence| fence.components = seventeen(&fence.components[0]),
	"components"
)]
#[case::a_path_that_climbs(
	|fence: &mut Fence| fence.components[1].path = String::from("../notify-api"),
	"components[1].path"
)]
#[case::a_path_inside_langdrift(
	|fence: &mut Fence| {
		fence.components[1].path = String::from(".langdrift/agent")
	},
	"components[1].path"
)]
#[case::a_repeated_path(
	|fence: &mut Fence| {
		fence.components[1].path = String::from("services/signup-api")
	},
	"components[1].path"
)]
#[case::a_stack_outside_the_set(
	|fence: &mut Fence| fence.components[2].stack = String::from("Kafka"),
	"components[2].stack"
)]
#[case::a_role_outside_the_set(
	|fence: &mut Fence| fence.components[2].role = String::from("queue"),
	"components[2].role"
)]
#[case::an_invalid_port(
	|fence: &mut Fence| fence.components[3].port = 70_000,
	"components[3].port"
)]
#[case::a_repeated_port(
	|fence: &mut Fence| fence.components[3].port = 8080,
	"components[3].port"
)]
#[case::an_invalid_image(
	|fence: &mut Fence| fence.components[4].image = String::from("signup image"),
	"components[4].image"
)]
#[case::a_repeated_image(
	|fence: &mut Fence| fence.components[4].image = fence.components[0].image.clone(),
	"components[4].image"
)]
#[case::no_cpu_share(
	|fence: &mut Fence| fence.components[0].cpu_millis = 0,
	"components[0].cpuMillis"
)]
#[case::no_memory_share(
	|fence: &mut Fence| fence.components[0].memory_mib = 0,
	"components[0].memoryMiB"
)]
#[case::two_replicas(
	|fence: &mut Fence| fence.components[0].replicas = 2,
	"components[0].replicas"
)]
#[case::cpu_shares_over_the_bound(
	|fence: &mut Fence| fence.cpu_millis = 1000,
	"components.cpuMillis"
)]
#[case::memory_shares_over_the_bound(
	|fence: &mut Fence| fence.memory_mib = 2048,
	"components.memoryMiB"
)]
fn each_rule_broken_alone_is_one_problem_on_its_field(
	#[case] break_rule: fn(&mut Fence),
	#[case] field: &str,
) {
	let mut fence = Fence::parse(&markdown()).unwrap();

	break_rule(&mut fence);

	let problems = fence.validate();

	assert_eq!(problems.len(), 1, "{problems:?}");
	assert_eq!(problems[0].field, field);
	assert!(!problems[0].rule.is_empty());
}

#[test]
fn validation_never_stops_at_the_first_problem() {
	let mut fence = Fence::parse(&markdown()).unwrap();

	fence.components[1].port = 8080;
	fence.cpu_millis = 1000;

	let mut fields = fence
		.validate()
		.into_iter()
		.map(|problem| problem.field)
		.collect::<Vec<_>>();

	fields.sort();

	assert_eq!(fields, ["components.cpuMillis", "components[1].port"]);
}

#[test]
fn a_problem_reads_as_core_words_it() {
	let mut fence = Fence::parse(&markdown()).unwrap();

	fence.components[2].stack = String::from("Kafka");
	fence.effort = String::from("max");

	let rules = fence
		.validate()
		.into_iter()
		.map(|problem| problem.rule)
		.collect::<Vec<_>>();

	assert_eq!(
		rules,
		[
			"must be one of low, medium, high",
			"must be one of Rust, Node.js, NATS, Redis, Next.js",
		]
	);
}
