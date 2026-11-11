//! The price list is read as `nsx init` laid it down

use std::path::PathBuf;

use nsx::command::{init::Init, plan::Plan};
use nsx::contract::cards::Cards;
use nsx::files::{layout::Layout, root::Root};
use nsx::money::{cents::Cents, error::Error, price::Price};
use nsx::session::kind::Kind;

use crate::money::listed::listed;
use crate::simulation::fake::recorder::Recorder;
use crate::simulation::fake::session::session;

#[tokio::test]
async fn the_price_list_reads_as_init_laid_it_down() {
	let project = tempfile::tempdir().unwrap();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	Init.execute(&mut session).await.unwrap();

	let prices = Price::load(&root, &Layout::under(&root)).unwrap();
	let models = prices
		.models
		.iter()
		.map(|model| (model.id.as_str(), model.input_cents, model.output_cents))
		.collect::<Vec<_>>();

	assert_eq!(prices.read_on, "2026-10-06");
	assert_eq!(prices.cache, "uncached");
	assert_eq!(
		models,
		[
			("claude-opus-5-5", Cents(400), Cents(2000)),
			("claude-sonnet-5-5", Cents(200), Cents(1000)),
			("claude-haiku-5-5", Cents(10), Cents(50)),
		]
	);
}

#[test]
fn a_project_without_a_price_list_is_a_stop() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Price::load(&root, &Layout::under(&root)).unwrap_err();

	assert!(matches!(refusal, Error::File(_)), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("prices.json"),
		"said {refusal}"
	);
}

#[test]
fn a_price_list_with_a_field_it_does_not_define_is_a_stop() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let written = r#"{"readOn": "2026-10-06", "cache": "uncached", "currency": "USD", "unit": "cents-per-million-tokens", "models": [], "discount": 10}"#;

	std::fs::create_dir_all(layout.prices.parent().unwrap()).unwrap();
	std::fs::write(&layout.prices, written).unwrap();

	let refusal = Price::load(&root, &layout).unwrap_err();

	assert!(matches!(refusal, Error::Prices { .. }), "was {refusal:?}");
	assert!(refusal.to_string().contains("discount"), "said {refusal}");
}

#[test]
fn a_price_list_with_a_negative_price_is_a_stop() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let written = r#"{"readOn": "2026-10-06", "cache": "uncached", "currency": "USD", "unit": "cents-per-million-tokens", "models": [{"id": "claude-opus-5-5", "name": "Claude Opus 5.5", "shortName": "Opus 5.5", "inputCents": 400, "outputCents": -2000}]}"#;

	std::fs::create_dir_all(layout.prices.parent().unwrap()).unwrap();
	std::fs::write(&layout.prices, written).unwrap();

	let refusal = Price::load(&root, &layout).unwrap_err();

	assert!(matches!(refusal, Error::Negative { .. }), "was {refusal:?}");
	assert!(
		refusal
			.to_string()
			.contains("claude-opus-5-5 has a negative price"),
		"said {refusal}"
	);
}

#[test]
fn a_price_list_with_a_model_beyond_the_three_is_a_stop() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let written = r#"{"readOn": "2026-10-06", "cache": "uncached", "currency": "USD", "unit": "cents-per-million-tokens", "models": [{"id": "claude-haiku-5-5", "name": "Claude Haiku 5.5", "shortName": "Haiku 5.5", "inputCents": 10, "outputCents": 50}, {"id": "claude-mini-1", "name": "Claude Mini 1", "shortName": "Mini 1", "inputCents": 1, "outputCents": 5}]}"#;

	std::fs::create_dir_all(layout.prices.parent().unwrap()).unwrap();
	std::fs::write(&layout.prices, written).unwrap();

	let refusal = Price::load(&root, &layout).unwrap_err();

	assert!(matches!(refusal, Error::Beyond { .. }), "was {refusal:?}");
	assert!(
		refusal
			.to_string()
			.contains("claude-mini-1 is not one of the three models nsx reads"),
		"said {refusal}"
	);
}

#[test]
fn a_price_list_that_lists_a_model_twice_is_a_stop() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let written = r#"{"readOn": "2026-10-06", "cache": "uncached", "currency": "USD", "unit": "cents-per-million-tokens", "models": [{"id": "claude-opus-5-5", "name": "Claude Opus 5.5", "shortName": "Opus 5.5", "inputCents": 400, "outputCents": 2000}, {"id": "claude-opus-5-5", "name": "Claude Opus 5.5", "shortName": "Opus 5.5", "inputCents": 1, "outputCents": 2}]}"#;

	std::fs::create_dir_all(layout.prices.parent().unwrap()).unwrap();
	std::fs::write(&layout.prices, written).unwrap();

	let refusal = Price::load(&root, &layout).unwrap_err();

	assert!(matches!(refusal, Error::Twice { .. }), "was {refusal:?}");
	assert!(
		refusal
			.to_string()
			.contains("it lists claude-opus-5-5 twice"),
		"said {refusal}"
	);
}

#[test]
fn the_plan_says_the_runtime_has_no_price_source() {
	let prices = Price {
		read_on: String::from("2026-10-06"),
		cache: String::from("uncached"),
		currency: String::from("USD"),
		unit: String::from("cents-per-million-tokens"),
		models: Vec::new(),
	};

	let line = prices.line();

	assert_eq!(line.id, "price");
	assert_eq!(line.kind, Kind::Output);
	assert_eq!(
		line.render(),
		"estimate   runtime has no price source · cpu and memory stay in millicores and MiB"
	);
}

#[test]
fn the_plan_cards_price_has_no_source_and_says_so_with_null() {
	let (prices, _forecast) = listed();
	let mut cards = Cards::default();

	prices.card(&mut cards);

	let written = serde_json::to_value(&cards).unwrap();

	assert_eq!(
		written["plan"]["price"],
		serde_json::json!({
			"source": null,
		})
	);
}

#[cfg(unix)]
#[test]
fn a_price_list_that_is_a_symbolic_link_is_a_stop() {
	let elsewhere = tempfile::tempdir().unwrap();
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let outside = elsewhere.path().join("prices.json");

	std::fs::write(&outside, "{}").unwrap();
	std::fs::create_dir_all(layout.prices.parent().unwrap()).unwrap();
	std::os::unix::fs::symlink(&outside, &layout.prices).unwrap();

	let refusal = Price::load(&root, &layout).unwrap_err();

	assert!(matches!(refusal, Error::File(_)), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("symbolic link"),
		"said {refusal}"
	);
}

#[tokio::test]
async fn a_plan_without_a_price_list_stops_before_its_first_line() {
	let project = tempfile::tempdir().unwrap();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let ticket = "# Book a demo\n\n**Status:** ready-for-agent\n\n- [prd](./0002-PRD-notes.prd.md)\n- [dd](./0005-DD-notes.dd.md)\n- [spec](./0011-SPEC-notes.spec.md)\n- [gr](/.langdrift/harness/guardrails/0001-finops.gr.md)\n";
	let design = "# Notes\n\n**cpuMillis:** 2000\n\n**memoryMiB:** 3072\n\n**Budget per run:** $25.00\n";
	let plan = Plan {
		ticket: String::from("0007-TICKET-notes"),
	};

	std::fs::create_dir_all(project.path().join(".langdrift")).unwrap();
	std::fs::create_dir_all(project.path().join(".agents")).unwrap();
	std::fs::write(
		project.path().join(".agents/0007-TICKET-notes.ticket.md"),
		ticket,
	)
	.unwrap();
	std::fs::write(project.path().join(".agents/0005-DD-notes.dd.md"), design)
		.unwrap();

	let stop = plan.execute(&mut session).await.unwrap_err();

	assert!(stop.sentence.contains("prices.json"), "said {stop:?}");
	assert!(recorder.lines.borrow().is_empty());
}
