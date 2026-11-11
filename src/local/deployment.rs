//! One running component
//!
//! A Deployment is one component of the applied `runtime.md` revision, running
//! in the run's space. There is one per component, and nothing else is
//! created for it.
//!
//! Definition only: the `Deployment` type: its name, its one replica, the
//! component's resolved image and port, and its cpu and memory. Its functions
//! are the files in `deployment/`.
//!
//! ## Rules
//!
//! - The name is the component's path with `/` replaced by `-`:
//!   `services-signup-api`, `services-notify-api`, `platform-nats`,
//!   `platform-redis` and `apps-signup` in the simulation.
//! - One replica, the fence's `replicas`, and one container that runs the
//!   resolved image on the component's port.
//! - Cpu and memory are both requested and limited at exactly the proposed
//!   share: `250m` and `256Mi` for `services/signup-api`.
//! - No Service stands between components, and no Secret or volume exists.
//!
//! Spec 0081.

pub mod manifest;
pub mod of;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deployment {
	pub name: String,
	pub image: String,
	pub port: i64,
	pub cpu_millis: i64,
	pub memory_mib: i64,
	pub replicas: i64,
}
