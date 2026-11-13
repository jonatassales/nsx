//! The manifest of a component
//!
//! The manifest is the file in a component's folder that says what the
//! component is. Five are known: `next.config.ts`, `Cargo.toml`,
//! `package.json`, `nats.conf` and `redis.conf`. The one the plan finds
//! decides the stack and the role, and the inspect line names it.
//!
//! Definition only: the `Manifest` type: which of the five files decided the
//! component. The five sit beside the type in the order the plan looks for
//! them. Its functions are the files in `manifest/`.
//!
//! ## Rules
//!
//! - Only these five are manifests. `pyproject.toml` and a `Dockerfile` are
//!   not.

pub mod classify;
pub mod code;
pub mod file;
pub mod find;
pub mod keyed;
pub mod port;
pub mod scripted;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Manifest {
	NextConfig,
	CargoToml,
	PackageJson,
	NatsConf,
	RedisConf,
}

pub const MANIFESTS: [Manifest; 5] = [
	Manifest::NextConfig,
	Manifest::CargoToml,
	Manifest::PackageJson,
	Manifest::NatsConf,
	Manifest::RedisConf,
];
