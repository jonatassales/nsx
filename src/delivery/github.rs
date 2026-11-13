//! The forge reached through `git` and `gh`
//!
//! The one forge nsx delivers to: the repository `git` works on and the
//! service `gh` serves. It starts and pushes the branch by running `git`,
//! and opens the draft pull request, reads its checks and marks it ready by
//! running `gh`.
//!
//! Definition only: the `Github` type, which is `Forge` as `git` and `gh`
//! serve it: the runner that starts them and the folder they start in, with
//! the variables they may see beside it. Its functions are the files in
//! `github/`.
//!
//! ## Reads and writes
//!
//! - runs `git` to start the branch, or to check it out when it exists
//! - runs `git push` of the branch to `origin`, with its upstream set
//! - runs `gh` to open a draft pull request
//! - runs `gh` to read the pull request's checks
//! - runs `gh` to mark the pull request ready
//!
//! ## Rules
//!
//! - When `gh` is missing or not signed in, the run stops before the pr line,
//!   with the branch pushed (the stop rule).
//! - The tools see the person's own sign-in, through the variables named
//!   here, and nsx never reads or prints it.
//! - Forges other than the one `gh` serves are out of scope.

pub mod checks;
pub mod forge;
pub mod open;
pub mod push;
pub mod ready;
pub mod start;

use std::path::PathBuf;

use crate::process::runner::Runner;

pub const PASSED: [&str; 5] = [
	"SSH_AUTH_SOCK",
	"GH_TOKEN",
	"GITHUB_TOKEN",
	"GH_HOST",
	"XDG_CONFIG_HOME",
];

pub struct Github<'run> {
	pub programs: &'run dyn Runner,
	pub directory: PathBuf,
}
