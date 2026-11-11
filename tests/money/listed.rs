//! The price list and the forecast `nsx init` lays down, read from the crate

use nsx::contract::decode::decode;
use nsx::money::{forecast::Forecast, price::Price};

pub fn listed() -> (Price, Forecast) {
	let prices = include_str!(concat!(
		env!("CARGO_MANIFEST_DIR"),
		"/langdrift/finops/prices.json"
	));
	let forecast = include_str!(concat!(
		env!("CARGO_MANIFEST_DIR"),
		"/langdrift/finops/forecasts.json"
	));

	(decode(prices).unwrap(), decode(forecast).unwrap())
}
