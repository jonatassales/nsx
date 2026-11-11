//! The machine's clock: a moment in UTC, to the second

use jiff::Timestamp;
use nsx::evidence::{clock::Clock, watch::Watch};

#[test]
fn the_clock_gives_this_moment_as_a_timestamp_in_utc() {
	let before = Timestamp::now().as_second();

	let moment = Watch.now();
	let read = moment.parse::<Timestamp>().unwrap();

	assert_eq!(moment.len(), "2026-11-11T09:40:00Z".len());
	assert!(moment.ends_with('Z'), "was {moment}");
	assert!(read.as_second() >= before, "was {moment}");
	assert!(read.as_second() <= Timestamp::now().as_second());
}
