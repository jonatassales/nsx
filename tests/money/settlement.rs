//! The settlement: what the run observed, and what it gives back

use nsx::money::{cents::Cents, settlement::Settlement};
use rstest::rstest;

use crate::money::listed::listed;

#[rstest]
#[case::sonnet_at_medium(1, 1140, 3_700_000, 240_000, 980, 160)]
#[case::haiku_at_low(2, 32, 2_100_000, 120_000, 27, 5)]
#[case::spent_to_the_cent(1, 980, 3_700_000, 240_000, 980, 0)]
#[case::spent_past_the_reservation(1, 900, 3_700_000, 240_000, 980, -80)]
#[case::nothing_used(1, 1140, 0, 0, 0, 1140)]
fn the_observed_spend_is_priced_and_the_rest_released(
	#[case] model: usize,
	#[case] reserved: i64,
	#[case] input: u64,
	#[case] output: u64,
	#[case] observed: i64,
	#[case] released: i64,
) {
	let (prices, _forecast) = listed();

	let settlement =
		Settlement::settle(Cents(reserved), &prices.models[model], input, output)
			.unwrap();

	assert_eq!(settlement.observed, Cents(observed));
	assert_eq!(settlement.released, Cents(released));
	assert_eq!(
		(settlement.input_tokens, settlement.output_tokens),
		(input, output)
	);
}

#[test]
fn an_amount_too_large_to_state_is_not_settled() {
	let (mut prices, _forecast) = listed();

	prices.models[0].input_cents = Cents(i64::MAX);

	let settlement =
		Settlement::settle(Cents(1140), &prices.models[0], u64::MAX, 0);

	assert_eq!(settlement, None);
}
