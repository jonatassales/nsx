//! The stacks
//!
//! The stack is what a component is built on, as its manifest decides it. The
//! plan prints it on the component's row and records it in the draft, and the
//! episode a run leaves keeps it (spec 0052).
//!
//! Definition only: the `Stack` type and its five values: `Go`, `Node.js`,
//! `NATS`, `Redis` and `Next.js`. Each carries its name, the ids of its
//! inspect line and of its row, and the port the stack usually serves on, if
//! it has one.
//!
//! ## Rules
//!
//! - The set is closed. There is no Python, no Docker and no unknown stack.
//! - Each value is spelled as the simulation shows it.
//! - NATS usually serves on 4222, Redis on 6379 and Next.js on 3000. Go and
//!   Node.js have no usual port.
//!
//! Spec 0079.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stack {
	pub name: &'static str,
	pub inspect: &'static str,
	pub row: &'static str,
	pub port: Option<i64>,
}

pub const GO: Stack = Stack {
	name: "Go",
	inspect: "inspectGo",
	row: "go",
	port: None,
};

pub const NODE: Stack = Stack {
	name: "Node.js",
	inspect: "inspectNode",
	row: "node",
	port: None,
};

pub const NATS: Stack = Stack {
	name: "NATS",
	inspect: "inspectNats",
	row: "nats",
	port: Some(4222),
};

pub const REDIS: Stack = Stack {
	name: "Redis",
	inspect: "inspectRedis",
	row: "redis",
	port: Some(6379),
};

pub const NEXT: Stack = Stack {
	name: "Next.js",
	inspect: "inspectNext",
	row: "next",
	port: Some(3000),
};
