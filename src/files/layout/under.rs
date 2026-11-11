//! The layout under a root
//!
//! One function: it takes the project's root and gives back the paths of
//! LangDrift's folder under it.
//!
//! ## Rules
//!
//! - It touches no file: a path is named whether or not it exists.
//!
//! Shared: used by every folder that reads or writes under `.langdrift/`.

use crate::files::{layout::Layout, root::Root};

impl Layout {
	pub fn under(root: &Root) -> Self {
		let langdrift = root.path.join(".langdrift");
		let finops = langdrift.join("finops");
		let harness = langdrift.join("harness");
		let microruntimes = langdrift.join("microruntimes");
		let state = langdrift.join("agent").join("state");

		Self {
			prices: finops.join("prices.json"),
			forecasts: finops.join("forecasts.json"),
			ledger: finops.join("ledger"),
			workflow: harness.join("workflow.json"),
			driver: harness.join("driver.json"),
			guardrails: harness.join("guardrails"),
			skills: langdrift.join("skills"),
			templates: langdrift.join("templates"),
			runtimes: microruntimes.join("runtimes"),
			episodes: microruntimes.join("episodes"),
			index: state.join("rag").join("index.json"),
			credentials: state.join("credentials").join("console.json"),
			ignored: state.join(".gitignore"),
			reported: state.join("console"),
			runs: state.join("runs"),
			kubeconfig: state.join("cluster").join("kubeconfig"),
			langdrift,
		}
	}
}
