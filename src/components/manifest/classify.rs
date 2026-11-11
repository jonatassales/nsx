//! Classifying a manifest
//!
//! One function: it takes a manifest and gives back the stack and the role it
//! decides.
//!
//! ## Rules
//!
//! - `next.config.ts` gives Next.js and web. `Cargo.toml` gives Rust and api.
//!   `package.json` gives Node.js and api. `nats.conf` gives NATS and broker.
//!   `redis.conf` gives Redis and cache.
//! - `package.json` never decides Next.js, and no combined stack exists.
//!
//! Spec 0079.

use crate::components::{
	manifest::Manifest,
	role::{API, BROKER, CACHE, Role, WEB},
	stack::{NATS, NEXT, NODE, REDIS, RUST, Stack},
};

impl Manifest {
	pub fn classify(&self) -> (Stack, Role) {
		match self {
			Self::NextConfig => (NEXT, WEB),
			Self::CargoToml => (RUST, API),
			Self::PackageJson => (NODE, API),
			Self::NatsConf => (NATS, BROKER),
			Self::RedisConf => (REDIS, CACHE),
		}
	}
}
