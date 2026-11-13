//! The session a command runs over
//!
//! What a command needs from outside itself, handed to it in one piece: the
//! folder the person typed the command in, the sink its lines go through,
//! the window it titles, the judge it asks and the resolver of images. The
//! golden test builds one over a copy of the fixture project, with a sink
//! that collects and fakes that answer; the binary builds one over the
//! working directory and the terminal.
//!
//! Definition only: the `Session` type, which carries the working directory,
//! the sink, the window, the judge, the image resolver, the console, the
//! runner of programs, the reader of a person's answers, the coding agent,
//! the clock, the cluster, the forge, the pace at which the forge is asked
//! for the checks, the pace at which a waiting run reports, the person's
//! interrupt, and the environment's variables,
//! which a command reads by name where it needs one.
//!
//! ## Rules
//!
//! - A command reaches nothing outside itself except through its session, so
//!   that the tests can put a fake in its place.

use std::path::PathBuf;
use std::time::Duration;

use crate::components::resolver::Resolver;
use crate::console::client::Client;
use crate::delivery::forge::Forge;
use crate::driver::agent::Agent;
use crate::evidence::clock::Clock;
use crate::flow::reader::Reader;
use crate::judgment::judge::Judge;
use crate::local::cluster::Cluster;
use crate::process::runner::Runner;
use crate::session::{interrupt::Interrupt, sink::Sink, window::Window};

pub struct Session<'run> {
	pub directory: PathBuf,
	pub sink: &'run dyn Sink,
	pub window: &'run dyn Window,
	pub judge: &'run dyn Judge,
	pub images: &'run dyn Resolver,
	pub console: &'run dyn Client,
	pub programs: &'run dyn Runner,
	pub answers: &'run dyn Reader,
	pub agent: &'run dyn Agent,
	pub clock: &'run dyn Clock,
	pub cluster: &'run dyn Cluster,
	pub forge: &'run dyn Forge,
	pub interval: Duration,
	pub beat: Duration,
	pub interrupt: &'run dyn Interrupt,
	pub environment: &'run dyn Fn(&str) -> Option<String>,
}
