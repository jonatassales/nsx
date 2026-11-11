//! An amount in cents prints as the simulation shows dollars

use nsx::money::cents::Cents;
use rstest::rstest;

#[rstest]
#[case::nothing(0, "$0.00")]
#[case::one_cent(1, "$0.01")]
#[case::under_a_dollar(99, "$0.99")]
#[case::the_released(160, "$1.60")]
#[case::the_estimate(950, "$9.50")]
#[case::the_observed(980, "$9.80")]
#[case::the_reservation(1140, "$11.40")]
#[case::the_budget(2500, "$25.00")]
#[case::the_estimate_at_opus_and_high(3400, "$34.00")]
#[case::the_reservation_over_the_budget(4080, "$40.80")]
#[case::a_thousand_dollars(100_000, "$1,000.00")]
#[case::thousands_grouped(123_456_789, "$1,234,567.89")]
#[case::below_zero(-160, "-$1.60")]
#[case::the_lowest(i64::MIN, "-$92,233,720,368,547,758.08")]
fn an_amount_prints_as_dollars_with_two_decimals(
	#[case] cents: i64,
	#[case] shown: &str,
) {
	assert_eq!(Cents(cents).format(), shown);
}

#[test]
fn an_amount_reads_and_writes_as_a_whole_number() {
	let amount = serde_json::from_str::<Cents>("1140").unwrap();

	assert_eq!(amount, Cents(1140));
	assert_eq!(serde_json::to_string(&amount).unwrap(), "1140");
	assert!(serde_json::from_str::<Cents>("11.4").is_err());
}
