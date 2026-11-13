//! The simulation's `runtime.md`: the fence sample in its block

use crate::contract::sample::sample;

pub fn markdown() -> String {
	let fence = sample("configuration/runtime.sample.json");

	format!("# runtime\n\n```langdrift\n{}\n```\n", fence.trim())
}
