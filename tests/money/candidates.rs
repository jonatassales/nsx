//! The candidates: the models of the price list the team offers, each at
//! the efforts nsx knows that the team offers for it

use std::path::PathBuf;

use nsx::contract::{offer::Offer, offered::Offered, provider::Provider};
use nsx::money::{
	budget::Budget, candidates::Candidates, cents::Cents, error::Error,
	spend::Spend,
};
use rstest::rstest;

use crate::money::{listed::listed, offered::offered};

const SONNET: &str = "claude-sonnet-5-5";

const HAIKU: &str = "claude-haiku-5-5";

#[test]
fn a_project_with_no_connection_plans_with_every_model_and_effort() {
	let (prices, _forecast) = listed();

	let candidates = Candidates::of(&prices, None).unwrap();
	let pairs = candidates
		.models
		.iter()
		.map(|candidate| (candidate.model.id.as_str(), candidate.efforts.len()))
		.collect::<Vec<_>>();

	assert_eq!(pairs, [("claude-opus-5-5", 3), (SONNET, 3), (HAIKU, 3)]);
}

#[test]
fn an_offer_narrows_the_candidates_and_the_spend_rows() {
	let (prices, forecast) = listed();
	let mut offer = offered(&[HAIKU, SONNET], &["xhigh", "medium", "low"]);
	let budget = Budget {
		amount: Cents(2500),
		source: PathBuf::from(".agents/0005-DD-book-a-demo-runtime.dd.md"),
	};

	offer.providers.push(Offered {
		provider: Provider::Openai,
		models: vec![String::from("gpt-5")],
		efforts: vec![String::from("high")],
	});

	let candidates = Candidates::of(&prices, Some(&offer)).unwrap();
	let spend = Spend::build(&candidates, &forecast, &budget).unwrap();
	let rows = spend
		.pairs
		.iter()
		.map(|pair| (pair.model.as_str(), pair.effort.as_str()))
		.collect::<Vec<_>>();

	assert_eq!(
		rows,
		[
			(SONNET, "low"),
			(SONNET, "medium"),
			(HAIKU, "low"),
			(HAIKU, "medium"),
		]
	);
	assert!(candidates.holds(SONNET, "medium"));
	assert!(!candidates.holds(SONNET, "high"));
	assert!(!candidates.holds("gpt-5", "high"));
}

#[test]
fn a_model_offered_at_no_known_effort_is_no_candidate() {
	let (prices, _forecast) = listed();
	let mut offer = offered(&[SONNET], &["medium"]);

	offer.providers.push(Offered {
		provider: Provider::Openai,
		models: vec![String::from(HAIKU)],
		efforts: vec![String::from("minimal")],
	});

	let candidates = Candidates::of(&prices, Some(&offer)).unwrap();

	assert_eq!(candidates.models.len(), 1);
	assert_eq!(candidates.models[0].model.id, SONNET);
}

#[rstest]
#[case::no_provider(Offer {
	providers: Vec::new(),
})]
#[case::no_model_nsx_prices(Offer {
	providers: vec![Offered {
		provider: Provider::Openai,
		models: vec![String::from("gpt-5")],
		efforts: vec![String::from("medium")],
	}],
})]
fn an_offer_of_no_model_nsx_prices_is_a_stop(#[case] offer: Offer) {
	let (prices, _forecast) = listed();

	let refusal = Candidates::of(&prices, Some(&offer)).unwrap_err();

	assert!(matches!(refusal, Error::Unoffered), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("Settings → AI Models"),
		"said {refusal}"
	);
}

#[test]
fn an_offer_at_no_effort_nsx_knows_is_a_stop() {
	let (prices, _forecast) = listed();
	let offer = offered(&[SONNET, HAIKU], &["max"]);

	let refusal = Candidates::of(&prices, Some(&offer)).unwrap_err();

	assert!(matches!(refusal, Error::Effortless), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("Settings → AI Models"),
		"said {refusal}"
	);
}

#[test]
fn a_price_list_missing_a_model_is_a_stop() {
	let (mut prices, _forecast) = listed();

	prices.models.retain(|model| model.id != SONNET);

	let refusal = Candidates::of(&prices, None).unwrap_err();

	assert!(matches!(refusal, Error::Model { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		"The price list has no model claude-sonnet-5-5: nsx cannot price a run with it."
	);
}
