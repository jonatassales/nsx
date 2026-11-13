//! nsx, the LangDrift CLI
//!
//! The binary a person types: `nsx init`, `nsx agent plan <ticket>` and
//! `nsx agent run`. This header is the crate's architecture document; the
//! behaviour is defined by the simulation on the website's home (ADR 0005).
//!
//! ## Shape
//!
//! One library and this thin binary. The library holds everything; the
//! binary parses the command line, builds the session over the working
//! directory and exits with what the command gives back. It stops a command
//! the person interrupts, except a run, which watches for the interrupt
//! itself so as to take its runtime down first (`command/run/halt.rs`). The
//! tests reach the library the same way, with a session of their own.
//!
//! ## Folders
//!
//! A folder under `src/` owns one part of the session. `command` holds the
//! three commands; the plan and the run are scripts there, each step calling
//! the folder that owns its line. `session` fixes how a line prints and how a
//! command stops. `contract` holds every shape a file or the console sees.
//! Every other folder is one element of the simulation, and `files` and
//! `process` are the machinery they share.
//!
//! ## Files
//!
//! A type's definition is one file, `gate.rs`. Each of its functions is one
//! file in the folder beside it, `gate/open.rs`, holding one `impl` block
//! with one function. Under a type, `line.rs` is the simulation line that
//! type owns. Every file opens with a preface that says what it holds.
//!
//! ## Rules that shape the code
//!
//! - Standard output is always a prefix of the simulation. A failure the
//!   simulation does not show is a stop: one sentence on standard error,
//!   exit 1 (`session/stop.rs`).
//! - A command reaches the outside only through its session, so that every
//!   outside system has a fake in the tests.
//! - Every test lives under `tests/`, never under `src/`.

#![deny(clippy::unwrap_used, clippy::expect_used)]

use std::cell::Cell;
use std::io::IsTerminal;
use std::process::ExitCode;

use clap::Parser;
use clap::error::ErrorKind;

use nsx::checks::suite::INTERVAL;
use nsx::command::{agent::Agent, cli::Cli, session::Session};
use nsx::components::derived::Derived;
use nsx::console::play::BEAT;
use nsx::console::{
	client::Client as Console, connection::Connection, offline::Offline,
	remote::Remote,
};
use nsx::delivery::github::Github;
use nsx::driver::launcher::Launcher;
use nsx::evidence::watch::Watch;
use nsx::files::{layout::Layout, root::Root};
use nsx::flow::keyboard::Keyboard;
use nsx::judgment::{absent::Absent, client::Client as Asker, judge::Judge};
use nsx::local::kind::Kind;
use nsx::process::local::Local;
use nsx::session::{
	interrupt::Interrupt, signal::Signal, stop::Stop, terminal::Terminal,
};

#[tokio::main]
async fn main() -> ExitCode {
	let cli = match Cli::try_parse() {
		Ok(cli) => cli,
		Err(refusal) if refusal.kind() == ErrorKind::DisplayHelp => {
			return match refusal.print() {
				Ok(()) => ExitCode::SUCCESS,
				Err(cause) => Stop::from(cause).exit(),
			};
		}
		Err(refusal) => return Stop::usage(&refusal).exit(),
	};

	let directory = match std::env::current_dir() {
		Ok(directory) => directory,
		Err(failure) => {
			let stop = Stop {
				sentence: format!(
					"nsx could not tell which folder it was typed in: {failure}."
				),
			};

			return stop.exit();
		}
	};

	let terminal = Terminal {
		toned: std::io::stdout().is_terminal(),
	};

	// A project that recorded a connection plans and runs connected. Outside a
	// project there is none to read: the command says so itself.
	let recorded = match Root::find(&directory) {
		Ok(root) => Connection::load(&root, &Layout::under(&root)),
		Err(_outside) => Ok(None),
	};
	let connection = match recorded {
		Ok(connection) => connection,
		Err(refusal) => return Stop::from(refusal).exit(),
	};
	let remote = connection.clone().map(Remote::open).transpose();
	let asker = connection.map(Asker::open).transpose();
	let (remote, asker) = match (remote, asker) {
		(Ok(remote), Ok(asker)) => (remote, asker),
		(Err(refusal), _asker) => return Stop::from(refusal).exit(),
		(_remote, Err(refusal)) => return Stop::from(refusal).exit(),
	};

	let console: &dyn Console = match &remote {
		Some(remote) => remote,
		None => &Offline,
	};
	let judge: &dyn Judge = match &asker {
		Some(asker) => asker,
		None => &Absent,
	};
	// The interrupt is watched from here on, so that one that arrives before
	// a command waits for it is not lost.
	let signal = match Signal::watch() {
		Ok(signal) => signal,
		Err(unwatched) => return Stop::from(unwatched).exit(),
	};
	let kind = Kind {
		programs: &Local,
		directory: directory.clone(),
		reached: Cell::new(false),
	};
	let github = Github {
		programs: &Local,
		directory: directory.clone(),
	};
	let delivering = matches!(cli, Cli::Agent(Agent::Run(_)));
	let mut session = Session {
		directory,
		sink: &terminal,
		window: &terminal,
		judge,
		images: &Derived,
		console,
		programs: &Local,
		answers: &Keyboard,
		agent: &Launcher {
			programs: &Local,
		},
		clock: &Watch,
		cluster: &kind,
		forge: &github,
		interval: INTERVAL,
		beat: BEAT,
		interrupt: &signal,
		environment: &|name| {
			std::env::var_os(name).map(|value| value.to_string_lossy().into_owned())
		},
	};

	// The interrupt is watched first, so it is always watched: a Ctrl-C that
	// arrives while a step holds the thread is a stop at the step's end.
	// A run watches for the interrupt itself, through its session: it has a
	// runtime to take down and a record to close. Any other command is
	// stopped where it stands.
	if delivering {
		return match cli.execute(&mut session).await {
			Ok(()) => ExitCode::SUCCESS,
			Err(stop) => stop.exit(),
		};
	}

	let ended = tokio::select! {
		biased;

		() = signal.wait() => Err(Stop::interrupted()),
		ended = cli.execute(&mut session) => ended,
	};

	match ended {
		Ok(()) => ExitCode::SUCCESS,
		Err(stop) => stop.exit(),
	}
}
