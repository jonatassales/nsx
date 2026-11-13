//! The roles
//!
//! The role is the part a component plays in the runtime. The manifest decides
//! it together with the stack, and the plan prints it beside the stack.
//!
//! Definition only: the `Role` type and its four values: `api`, `broker`,
//! `cache` and `web`.
//!
//! ## Rules
//!
//! - The set is closed. There is no worker role.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Role {
	pub name: &'static str,
}

pub const API: Role = Role {
	name: "api",
};

pub const BROKER: Role = Role {
	name: "broker",
};

pub const CACHE: Role = Role {
	name: "cache",
};

pub const WEB: Role = Role {
	name: "web",
};
