//! The estimate, the reservation and the gate: the run's money before it works

use nsx::money::{
	cents::Cents, estimate::Estimate, gate::Gate, reservation::Reservation,
	tokens::Tokens,
};
use nsx::session::kind::Kind;
use rstest::rstest;

use crate::money::listed::listed;

#[rstest]
#[case::sonnet_at_medium(1, 3_500_000, 250_000, 950)]
#[case::opus_at_high(0, 6_000_000, 500_000, 3400)]
#[case::haiku_at_low(2, 2_000_000, 120_000, 26)]
#[case::half_a_cent_rounds_up(2, 50_000, 0, 1)]
#[case::under_half_a_cent_rounds_down(2, 49_999, 0, 0)]
#[case::no_tokens(0, 0, 0, 0)]
fn tokens_are_priced_per_million_and_rounded_half_up(
	#[case] model: usize,
	#[case] input: u64,
	#[case] output: u64,
	#[case] cents: i64,
) {
	let (prices, _forecast) = listed();

	let amount = Estimate::compute(&prices.models[model], input, output);

	assert_eq!(amount, Some(Cents(cents)));
}

#[test]
fn an_amount_too_large_for_cents_is_not_stated() {
	let (mut prices, _forecast) = listed();

	prices.models[0].input_cents = Cents(i64::MAX);

	assert_eq!(Estimate::compute(&prices.models[0], u64::MAX, 0), None);
}

#[rstest]
#[case::sonnet_at_medium(950, 1140)]
#[case::opus_at_high(3400, 4080)]
#[case::haiku_at_low(26, 32)]
#[case::haiku_at_high(85, 102)]
#[case::nothing(0, 0)]
fn the_reservation_is_the_estimate_plus_a_fifth_rounded_up(
	#[case] estimate: i64,
	#[case] reserved: i64,
) {
	let reservation = Reservation::total(Cents(estimate)).unwrap();

	assert_eq!(reservation.amount, Cents(reserved));
}

#[rstest]
#[case::sonnet_at_medium(
	950,
	(3500, 250),
	"console › estimate   $9.50 · 3.5M input · 250K output · list prices · FIN-02"
)]
#[case::haiku_at_low(
	26,
	(2000, 120),
	"console › estimate   $0.26 · 2M input · 120K output · list prices · FIN-02"
)]
#[case::opus_at_high(
	3400,
	(6000, 500),
	"console › estimate   $34.00 · 6M input · 500K output · list prices · FIN-02"
)]
#[case::every_digit_of_a_million_is_kept(
	1,
	(1050, 1999),
	"console › estimate   $0.01 · 1.05M input · 1.999M output · list prices · FIN-02"
)]
#[case::the_edges_of_a_thousand_and_a_million(
	1,
	(999, 1000),
	"console › estimate   $0.01 · 999K input · 1M output · list prices · FIN-02"
)]
#[case::none_and_many(
	1,
	(0, 12500),
	"console › estimate   $0.01 · 0K input · 12.5M output · list prices · FIN-02"
)]
fn the_run_states_its_estimate_and_the_expected_tokens(
	#[case] amount: i64,
	#[case] tokens: (u32, u32),
	#[case] printed: &str,
) {
	let estimate = Estimate {
		model: String::from("claude-sonnet-5-5"),
		effort: String::from("medium"),
		tokens: Tokens {
			input: tokens.0,
			output: tokens.1,
		},
		amount: Cents(amount),
	};

	let line = estimate.line();

	assert_eq!(line.id, "estimate");
	assert_eq!(line.kind, Kind::Output);
	assert_eq!(line.render(), printed);
}

#[test]
fn a_reservation_inside_the_budget_opens_the_gate() {
	let gate = Gate::open(Cents(1140), Cents(2500));
	let lines = gate.line();

	assert_eq!(lines.len(), 1);
	assert_eq!(
		(lines[0].id.as_str(), lines[0].kind),
		("gateOpened", Kind::Success)
	);
	assert_eq!(
		lines[0].render(),
		"console › gate       reserved $11.40 of $25.00 · opened"
	);
}

#[test]
fn a_reservation_equal_to_the_budget_opens_the_gate() {
	let gate = Gate::open(Cents(2500), Cents(2500));

	assert!(matches!(gate, Gate::Opened { .. }));
}

#[test]
fn a_reservation_over_the_budget_pauses_with_a_decision_to_make() {
	let gate = Gate::open(Cents(4080), Cents(2500));
	let lines = gate.line();
	let printed = lines.iter().map(|line| line.render()).collect::<Vec<_>>();

	assert!(matches!(gate, Gate::Paused { .. }));
	assert_eq!(
		(lines[0].id.as_str(), lines[0].kind),
		("gatePaused", Kind::Warning)
	);
	assert_eq!(
		(lines[1].id.as_str(), lines[1].kind),
		("paused", Kind::Comment)
	);
	assert_eq!(
		printed,
		[
			"console › gate       reservation $40.80 exceeds the $25.00 budget · paused",
			"# needs a decision: raise the budget or lower the effort · nothing was paid for",
		]
	);
}
