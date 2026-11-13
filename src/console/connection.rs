//! The connection to the console
//!
//! What a project needs to reach the console: the secret a team member
//! generated on the console's Settings → Security → Secrets, the console's
//! origin, `https://langdrift.md` unless the team names another.
//! The judge and the reports use this one connection.
//!
//! Definition only: the `Connection` type, which carries the console's
//! origin and the secret. The default origin and the two environment
//! variables' names sit beside the type. Its functions are the files in
//! `connection/`.
//!
//! ## Rules
//!
//! - The secret is never printed, logged or written into evidence.
//! - The secret and the origin live in the private credentials file,
//!   `.langdrift/agent/state/credentials/console.json`: mode 0600, in a
//!   folder of mode 0700, refused if Git tracks it.
//! - The origin is `https://langdrift.md` unless `MCP_CONSOLE_URL` names
//!   another when `nsx init` records the connection (owner, 2026-11-11).
//! - The runtime's id comes in the console's answer to the runtime's first
//!   report. nsx keeps it in the project's private, git-ignored state
//!   (`report.rs`) and sends it with every later report of that runtime.

pub mod load;
pub mod record;

use serde::{Deserialize, Serialize};

pub const ORIGIN: &str = "https://langdrift.md";

pub const SECRET_VARIABLE: &str = "LANGDRIFT_SECRET";

pub const ORIGIN_VARIABLE: &str = "MCP_CONSOLE_URL";

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Connection {
	pub origin: String,
	pub secret: String,
}
