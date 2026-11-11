//! The simulation's suggestion: Sonnet 5.5 at medium

use nsx::judgment::suggestion::Suggestion;

pub fn suggested() -> Suggestion {
	Suggestion {
		model: String::from("claude-sonnet-5-5"),
		effort: String::from("medium"),
		effort_reason: String::from("low is too little, high too much"),
		model_reason: String::from("Haiku 5.5 is too small, Opus 5.5 too much"),
	}
}
