//! The driver configuration
//!
//! How the team's coding agent is started, as one plain file the team can
//! read and version. It names the agent's command and arguments, the
//! variables passed through to it, and where its JSON output holds the token
//! counts.
//!
//! Definition only: the `Config` type, which carries `executable`, `args`,
//! `passEnv`, and `usage`, a `Paths` with `inputTokens` and `outputTokens`.
//! Its functions are the files in `config/`.
//!
//! ## Rules
//!
//! - `args` may hold the placeholders `{{prompt}}`, `{{model}}`,
//!   `{{effort}}`, `{{report}}`, `{{skill}}`, `{{stage}}`, `{{ticket}}` and
//!   `{{run}}`. Each is filled per argument and never splits an argument.
//! - `args` must carry `{{model}}` and `{{effort}}`.
//! - `passEnv` names the variables passed through from nsx's environment.
//! - The two usage entries are dotted paths into the agent's JSON output.
//! - No placeholder carries a lap: a stage is one call.
//!
//! ## Open
//!
//! - Whether `nsx init` writes a working `driver.json` for one coding agent,
//!   and which. Until the owner decides, the file is the team's to fill.

pub mod load;

use serde::Deserialize;

use crate::driver::paths::Paths;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Config {
	pub executable: String,
	pub args: Vec<String>,
	pub pass_env: Vec<String>,
	pub usage: Paths,
}
