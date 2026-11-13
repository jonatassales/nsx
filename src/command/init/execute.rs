//! Creating `.langdrift/`
//!
//! One function: it takes the working directory and writes `.langdrift/`
//! there, with exactly the files below. It prints nothing and exits 0.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/skills/`, one folder per skill
//! - writes `.langdrift/finops/prices.json`
//! - writes `.langdrift/finops/forecasts.json`
//! - writes `.langdrift/harness/workflow.json`
//! - writes `.langdrift/harness/guardrails/0001-finops.gr.md`
//! - writes `.langdrift/templates/`: `ticket.md`, `prd.md`, `dd.md`, `spec.md`
//! - environment: `LANGDRIFT_SECRET` and `MCP_CONSOLE_URL`
//! - when the environment offers a secret: records the console's connection
//!   (`console/connection/record.rs`)
//!
//! ## Rules
//!
//! - Each file's content belongs to the folder that reads it. Another init
//!   file joins only when a folder reads it.
//! - An existing file is kept byte for byte. Running it twice changes nothing.
//! - `harness/driver.json` is not written: the team writes it.
//! - The content is built into the binary from `langdrift/`, beside `src/`.
//! - It never writes through a symbolic link: a `.langdrift/` that is one is
//!   refused.
//! - Without `LANGDRIFT_SECRET` nothing is recorded and nothing is sent.
//! - It writes no project identity, input record, mirror, budget, gate,
//!   artifacts folder record, execution rule or CI workflow.
//!
//! ## Open
//!
//! - The document templates. The plan and the run never read them. Until the
//!   owner decides, they are written under `.langdrift/templates/`.

use std::path::Path;

use crate::command::{init::Init, session::Session};
use crate::console::connection::{
	Connection, ORIGIN, ORIGIN_VARIABLE, SECRET_VARIABLE,
};
use crate::files::{
	error::Error, layout::Layout, root::Root, within::within, write::write,
};
use crate::session::stop::Stop;

// One file `nsx init` lays down: its path under `.langdrift/`, and the file of
// `langdrift/`, beside `src/`, that holds its content. The repository's naming
// rules give every Markdown file a type suffix, so the guardrail and the
// templates carry one there and take the client's name when laid down.
macro_rules! asset {
	($name:literal) => {
		asset!($name, $name)
	};
	($laid:literal, $held:literal) => {
		(
			$laid,
			include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/langdrift/", $held)),
		)
	};
}

const ASSETS: [(&str, &str); 16] = [
	asset!("finops/prices.json"),
	asset!("finops/forecasts.json"),
	asset!("harness/workflow.json"),
	asset!(
		"harness/guardrails/0001-finops.gr.md",
		"harness/guardrails/0001-finops.policy.md"
	),
	asset!("skills/grilling/SKILL.md"),
	asset!("skills/to-spec/SKILL.md"),
	asset!("skills/to-tickets/SKILL.md"),
	asset!("skills/implement/SKILL.md"),
	asset!("skills/tdd/SKILL.md"),
	asset!("skills/tdd/mocking.guide.md"),
	asset!("skills/tdd/tests.guide.md"),
	asset!("skills/code-review/SKILL.md"),
	asset!("templates/ticket.md", "templates/empty.ticket.md"),
	asset!("templates/prd.md", "templates/empty.prd.md"),
	asset!("templates/dd.md", "templates/empty.tdd.md"),
	asset!("templates/spec.md", "templates/empty.spec.md"),
];

impl Init {
	pub async fn execute(&self, session: &mut Session<'_>) -> Result<(), Stop> {
		let root = Root {
			path: session.directory.clone(),
		};

		for (name, content) in ASSETS {
			let path = within(&root, &Path::new(".langdrift").join(name))?;

			let kept = path.try_exists().map_err(|cause| Error::Unreadable {
				path: path.clone(),
				cause,
			})?;

			// A file already there is the team's: it is kept byte for byte.
			if kept {
				continue;
			}

			write(&path, content.as_bytes())?;
		}

		// The environment's variables are read here, where they are recorded.
		let Some(secret) = (session.environment)(SECRET_VARIABLE) else {
			return Ok(());
		};
		let stated = (session.environment)(ORIGIN_VARIABLE);
		let connection = Connection {
			origin: stated.unwrap_or_else(|| String::from(ORIGIN)),
			secret,
		};
		let layout = Layout::under(&root);

		connection.record(&root, &layout, session.programs).await?;

		Ok(())
	}
}
