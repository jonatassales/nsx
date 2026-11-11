//! The spend table: one row per model and effort, priced against the budget

use std::path::PathBuf;

use nsx::contract::cards::Cards;
use nsx::money::{
	budget::Budget, cents::Cents, error::Error, spend::Spend, tokens::Tokens,
};

use crate::money::listed::listed;

const DESIGN: &str = ".agents/0005-DD-book-a-demo-runtime.dd.md";

#[test]
fn the_nine_rows_are_the_figures_of_the_simulation() {
	let (prices, forecast) = listed();
	let budget = Budget {
		amount: Cents(2500),
		source: PathBuf::from(DESIGN),
	};

	let spend = Spend::build(&prices, &forecast, &budget).unwrap();
	let rows = spend
		.pairs
		.iter()
		.map(|pair| {
			(
				pair.model.as_str(),
				pair.effort.as_str(),
				pair.estimate.0,
				pair.reserved.0,
				pair.over,
			)
		})
		.collect::<Vec<_>>();

	assert_eq!(spend.budget, budget);
	assert_eq!(spend.margin, 20);
	assert_eq!(
		rows,
		[
			("claude-opus-5-5", "low", 1040, 1248, false),
			("claude-opus-5-5", "medium", 1900, 2280, false),
			("claude-opus-5-5", "high", 3400, 4080, true),
			("claude-sonnet-5-5", "low", 520, 624, false),
			("claude-sonnet-5-5", "medium", 950, 1140, false),
			("claude-sonnet-5-5", "high", 1700, 2040, false),
			("claude-haiku-5-5", "low", 26, 32, false),
			("claude-haiku-5-5", "medium", 48, 58, false),
			("claude-haiku-5-5", "high", 85, 102, false),
		]
	);
}

#[test]
fn each_row_carries_the_expected_tokens_of_its_effort() {
	let (prices, forecast) = listed();
	let budget = Budget {
		amount: Cents(2500),
		source: PathBuf::from(DESIGN),
	};

	let spend = Spend::build(&prices, &forecast, &budget).unwrap();
	let tokens = spend
		.pairs
		.iter()
		.take(3)
		.map(|pair| (pair.tokens.input, pair.tokens.output))
		.collect::<Vec<_>>();

	assert_eq!(tokens, [(2000, 120), (3500, 250), (6000, 500)]);
}

#[test]
fn a_reservation_equal_to_the_budget_is_not_over_it() {
	let (prices, forecast) = listed();
	let at = Budget {
		amount: Cents(2280),
		source: PathBuf::from(DESIGN),
	};
	let under = Budget {
		amount: Cents(2279),
		source: PathBuf::from(DESIGN),
	};

	let equal = Spend::build(&prices, &forecast, &at).unwrap();
	let above = Spend::build(&prices, &forecast, &under).unwrap();

	assert!(!equal.pairs[1].over);
	assert!(above.pairs[1].over);
}

#[test]
fn half_a_cent_rounds_up_and_the_reservation_rounds_up() {
	let (prices, mut forecast) = listed();
	let budget = Budget {
		amount: Cents(2500),
		source: PathBuf::from(DESIGN),
	};
	let low = Tokens {
		input: 50,
		output: 0,
	};

	// Haiku: 50 thousand input tokens at 10 cents a million is half a cent.
	forecast.by_effort.insert(String::from("low"), low);

	let spend = Spend::build(&prices, &forecast, &budget).unwrap();
	let haiku = &spend.pairs[6];

	assert_eq!(haiku.estimate, Cents(1));
	assert_eq!(haiku.reserved, Cents(2));
}

#[test]
fn a_price_list_missing_a_model_is_a_stop() {
	let (mut prices, forecast) = listed();
	let budget = Budget {
		amount: Cents(2500),
		source: PathBuf::from(DESIGN),
	};

	prices
		.models
		.retain(|model| model.id != "claude-sonnet-5-5");

	let refusal = Spend::build(&prices, &forecast, &budget).unwrap_err();

	assert!(matches!(refusal, Error::Model { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		"The price list has no model claude-sonnet-5-5: nsx cannot price a run with it."
	);
}

#[test]
fn a_forecast_missing_an_effort_is_a_stop() {
	let (prices, mut forecast) = listed();
	let budget = Budget {
		amount: Cents(2500),
		source: PathBuf::from(DESIGN),
	};

	forecast.by_effort.remove("high");

	let refusal = Spend::build(&prices, &forecast, &budget).unwrap_err();

	assert!(matches!(refusal, Error::Effort { .. }), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("effort high"),
		"said {refusal}"
	);
}

#[test]
fn an_amount_too_large_for_cents_is_a_stop() {
	let (mut prices, mut forecast) = listed();
	let budget = Budget {
		amount: Cents(2500),
		source: PathBuf::from(DESIGN),
	};
	let high = Tokens {
		input: u32::MAX,
		output: u32::MAX,
	};

	prices.models[0].input_cents = Cents(i64::MAX);
	forecast.by_effort.insert(String::from("high"), high);

	let refusal = Spend::build(&prices, &forecast, &budget).unwrap_err();

	assert!(matches!(refusal, Error::Unpriced { .. }), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("claude-opus-5-5"),
		"said {refusal}"
	);
}

#[test]
fn the_workflow_section_carries_the_budget_the_margin_and_nine_rows() {
	let (prices, forecast) = listed();
	let budget = Budget {
		amount: Cents(2500),
		source: PathBuf::from(DESIGN),
	};
	let spend = Spend::build(&prices, &forecast, &budget).unwrap();
	let mut cards = Cards::default();

	spend.card(&mut cards);

	let written = serde_json::to_value(&cards).unwrap();
	let workflow = &written["workflow"];

	assert_eq!(workflow["budgetCents"], 2500);
	assert_eq!(workflow["budgetSource"], DESIGN);
	assert_eq!(workflow["marginPercent"], 20);
	assert_eq!(workflow["spend"].as_array().unwrap().len(), 9);
	assert_eq!(
		workflow["spend"][2],
		serde_json::json!({
			"model": "claude-opus-5-5",
			"effort": "high",
			"inputThousands": 6000,
			"outputThousands": 500,
			"estimateCents": 3400,
			"reservedCents": 4080,
			"overBudget": true,
		})
	);
}
