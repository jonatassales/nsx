//! The simulation's `runtime.md`: Core's fence sample in its block

use crate::contract::sample::sample;

pub fn markdown() -> String {
	let fence = sample("configuration/runtime.sample.json");

	format!("# runtime\n\n```langdrift\n{}\n```\n", fence.trim())
}
