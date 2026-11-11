//! Safe Seat's verdict, met or not

use nsx::seat::verdict::Verdict;

pub fn decided(met: bool) -> Verdict {
	Verdict {
		met,
		decided_at: String::from("2026-11-11T09:40:00Z"),
	}
}
