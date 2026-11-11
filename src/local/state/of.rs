//! The runtime that is ready
//!
//! One function: it takes the applied intent and the Deployments that
//! started, and gives back the runtime's state, ready.
//!
//! ## Rules
//!
//! - The runtime's id and the revision are the applied intent's. The totals
//!   are the sum of the started Deployments' shares.
//!
//! Spec 0081.

use crate::contract::intent::Intent;
use crate::local::{deployment::Deployment, state::State};

impl State {
	pub fn of(intent: &Intent, deployments: &[Deployment]) -> Self {
		let sum = |share: fn(&Deployment) -> i64| {
			deployments
				.iter()
				.map(share)
				.fold(0_i64, |total, part| total.saturating_add(part))
		};

		Self {
			runtime: intent.runtime_id.clone(),
			revision: intent.revision,
			deployments: deployments.len() as u64,
			cpu_millis: sum(|deployment| deployment.cpu_millis),
			memory_mib: sum(|deployment| deployment.memory_mib),
			removed: None,
		}
	}
}
