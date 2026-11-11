//! Classifying a manifest
//!
//! One function: it takes a manifest and gives back the stack and the role it
//! decides.
//!
//! ## Rules
//!
//! - `next.config.ts` gives Next.js and web. `go.mod` gives Go and api.
//!   `package.json` gives Node.js and api. `nats.conf` gives NATS and broker.
//!   `redis.conf` gives Redis and cache.
//! - `package.json` never decides Next.js, and no combined stack exists.
//!
//! Spec 0079.

use crate::components::{
	manifest::Manifest,
	role::{API, BROKER, CACHE, Role, WEB},
	stack::{GO, NATS, NEXT, NODE, REDIS, Stack},
};

impl Manifest {
	pub fn classify(&self) -> (Stack, Role) {
		match self {
			Self::NextConfig => (NEXT, WEB),
			Self::GoMod => (GO, API),
			Self::PackageJson => (NODE, API),
			Self::NatsConf => (NATS, BROKER),
			Self::RedisConf => (REDIS, CACHE),
		}
	}
}
