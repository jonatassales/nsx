//! A manifest's file name
//!
//! One function: it gives back the name of the file a manifest is, as it sits
//! in the component's folder and as the inspect line names it.
//!
//! Spec 0079.

use crate::components::manifest::Manifest;

impl Manifest {
	pub fn file(&self) -> &'static str {
		match self {
			Self::NextConfig => "next.config.ts",
			Self::GoMod => "go.mod",
			Self::PackageJson => "package.json",
			Self::NatsConf => "nats.conf",
			Self::RedisConf => "redis.conf",
		}
	}
}
