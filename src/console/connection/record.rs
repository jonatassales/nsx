//! Recording the connection
//!
//! One function: it takes the project's root, the paths under `.langdrift/`
//! and the runner of programs, and records the connection in the project's
//! private credentials file.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/agent/state/credentials/console.json`, mode 0600, in
//!   a folder of mode 0700
//! - writes `.langdrift/agent/state/.gitignore`, so that Git ignores the
//!   private state: `git add .` never takes the secret
//! - runs `git ls-files` to ask whether Git tracks that file
//!
//! ## Rules
//!
//! - `nsx init` calls it when `LANGDRIFT_SECRET` is set, and prints nothing
//!   more. When it is not set, nothing is recorded and nothing is sent.
//! - Nothing is sent here: the project's first report is its first plan's.
//! - The origin must be one nsx will send a secret to (`origin.rs`), and the
//!   secret printable text with no space.
//! - The credentials file is refused if Git tracks it, before anything is
//!   written. A project that is not a Git repository tracks nothing.
//! - The secret never enters a tracked file, a log or a printed line, and no
//!   refusal's sentence carries it.
//!
//! Spec 0081.

use crate::console::{connection::Connection, error::Error, origin::origin};
use crate::contract::encode::encode;
use crate::files::{
	hide::hide, inside::inside, layout::Layout, root::Root, sealed::sealed,
};
use crate::process::{
	environment::Environment, program::Program, runner::Runner,
};

impl Connection {
	pub async fn record(
		&self,
		root: &Root,
		layout: &Layout,
		runner: &dyn Runner,
	) -> Result<(), Error> {
		let file = inside(root, &layout.credentials)?;
		let Some(origin) = origin(&self.origin) else {
			return Err(Error::Origin);
		};
		let printable = !self.secret.is_empty()
			&& self.secret.chars().all(|letter| letter.is_ascii_graphic());

		if !printable {
			return Err(Error::Secret);
		}

		let listing = Program {
			name: String::from("git"),
			arguments: vec![
				String::from("ls-files"),
				String::from("--error-unmatch"),
				String::from("--"),
				file.to_string_lossy().into_owned(),
			],
			environment: Environment::allow(&[]),
			directory: root.path.clone(),
			input: Vec::new(),
		};

		// A machine without Git, or a project outside a repository, tracks
		// nothing: only Git saying it tracks the file refuses it.
		let listed = runner.run(&listing).await;
		let tracked = listed.is_ok_and(|listed| listed.status == Some(0));

		if tracked {
			return Err(Error::Tracked {
				path: file,
			});
		}

		let recorded = Self {
			origin,
			secret: self.secret.clone(),
		};
		let mut text = encode(&recorded).map_err(|cause| Error::Unwritten {
			path: file.clone(),
			cause,
		})?;

		text.push('\n');

		// The private state is ignored before the secret is written into it.
		hide(root, layout)?;

		Ok(sealed(&file, text.as_bytes())?)
	}
}
