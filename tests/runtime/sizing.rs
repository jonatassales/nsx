//! The sizing: the shares summed against the bound, and the headroom left

use nsx::contract::cards::Cards;
use nsx::runtime::{error::Error, sizing::Sizing};
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::runtime::{bound::bound, shared::shared};

#[test]
fn the_fixtures_shares_sum_inside_the_bound() {
	let components = [
		shared("services/signup-api", 250, 256),
		shared("services/notify-api", 300, 512),
		shared("platform/nats", 200, 256),
		shared("platform/redis", 200, 256),
		shared("apps/signup", 500, 1024),
	];

	let sizing = Sizing::sum(&components, &bound());
	let line = sizing.line();

	sizing.fit().unwrap();

	assert_eq!((sizing.cpu_millis, sizing.memory_mib), (1450, 2304));
	assert_eq!(
		(sizing.headroom_cpu_millis, sizing.headroom_memory_mib),
		(550, 768)
	);
	assert_eq!(sizing.deployments, 5);
	assert_eq!(sizing.bound, bound());
	assert_eq!(line.id, "size");
	assert_eq!(line.kind, Kind::Success);
	assert_eq!(
		line.render(),
		"✓ size       1450m of 2000m · 2304 of 3072 MiB · 5 one-replica Deployments · proposed shares"
	);
}

#[test]
fn a_sum_equal_to_the_bound_fits_with_no_headroom() {
	let components = [
		shared("services/a", 1500, 2048),
		shared("services/b", 500, 1024),
	];

	let sizing = Sizing::sum(&components, &bound());

	sizing.fit().unwrap();

	assert_eq!(
		(sizing.headroom_cpu_millis, sizing.headroom_memory_mib),
		(0, 0)
	);
}

#[rstest]
#[case::in_cpu(2001, 3072)]
#[case::in_memory(2000, 3073)]
#[case::in_both(4000, 8192)]
#[case::beyond_any_number(i64::MAX, 1)]
fn a_plan_over_the_bound_is_refused(
	#[case] cpu_millis: i64,
	#[case] memory_mib: i64,
) {
	let components = [
		shared("services/a", cpu_millis, memory_mib),
		shared("services/b", 0, 0),
	];

	let refusal = Sizing::sum(&components, &bound()).fit().unwrap_err();

	assert!(matches!(refusal, Error::Over { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		format!(
			"The components need {cpu_millis}m and {memory_mib} MiB, over the bound of 2000m and 3072 MiB the team set in .agents/0005-DD-book-a-demo-runtime.dd.md."
		)
	);
}

#[test]
fn two_shares_too_large_to_add_are_over_the_bound() {
	let components = [
		shared("services/a", i64::MAX, 1),
		shared("services/b", i64::MAX, 1),
	];

	let refusal = Sizing::sum(&components, &bound()).fit().unwrap_err();

	assert!(matches!(refusal, Error::Over { .. }), "was {refusal:?}");
}

#[test]
fn summing_never_refuses_and_the_headroom_is_not_clamped() {
	let components = [shared("services/a", 2500, 4096)];

	let sizing = Sizing::sum(&components, &bound());

	assert_eq!((sizing.cpu_millis, sizing.memory_mib), (2500, 4096));
	assert_eq!(
		(sizing.headroom_cpu_millis, sizing.headroom_memory_mib),
		(-500, -1024)
	);
}

#[test]
fn the_card_gets_the_totals_then_the_headroom_and_the_resource() {
	let components = [
		shared("services/signup-api", 250, 256),
		shared("services/notify-api", 300, 512),
	];
	let sizing = Sizing::sum(&components, &bound());
	let mut cards = Cards::default();

	sizing.proposed(&mut cards);

	let stepped = serde_json::to_value(&cards).unwrap();

	sizing.card(&mut cards);

	let sized = serde_json::to_value(&cards).unwrap();

	assert_eq!(
		stepped["plan"]["sizing"],
		serde_json::json!({
			"proposed": {
				"cpuMillis": 550,
				"memoryMiB": 768,
			},
		})
	);
	assert_eq!(
		sized["plan"]["sizing"]["headroom"],
		serde_json::json!({
			"cpuMillis": 1450,
			"memoryMiB": 2304,
		})
	);
	assert_eq!(
		sized["plan"]["sizing"]["resource"],
		serde_json::json!({
			"kind": "Deployment",
			"count": 2,
			"replicas": 1,
		})
	);
}
