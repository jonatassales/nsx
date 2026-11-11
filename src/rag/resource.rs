//! The workload an episode's run asked for
//!
//! The suggested resource of a past run, as its plan stated it.
//!
//! Definition only: the `Resource` type: the workload's kind, such as
//! `Deployment`, how many there were and the replicas of each.
//!
//! Spec 0078.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Resource {
	pub kind: String,
	pub count: u64,
	pub replicas: u64,
}
