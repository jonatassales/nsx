//! The price list `nsx init` lays down: three models at their list prices

use nsx::money::{cents::Cents, model::Model, price::Price};

pub fn prices() -> Price {
	let model = |id: &str, short: &str, input: i64, output: i64| Model {
		id: String::from(id),
		name: format!("Claude {short}"),
		short_name: String::from(short),
		input_cents: Cents(input),
		output_cents: Cents(output),
	};

	Price {
		read_on: String::from("2026-10-06"),
		cache: String::from("uncached"),
		currency: String::from("USD"),
		unit: String::from("cents-per-million-tokens"),
		models: vec![
			model("claude-opus-5-5", "Opus 5.5", 400, 2000),
			model("claude-sonnet-5-5", "Sonnet 5.5", 200, 1000),
			model("claude-haiku-5-5", "Haiku 5.5", 10, 50),
		],
	}
}
