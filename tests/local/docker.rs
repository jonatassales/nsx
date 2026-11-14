//! Docker, asked whether it answers and started when it does not

use std::collections::VecDeque;

use nsx::local::{error::Error, system::System};
use nsx::process::output::Output;

use crate::local::{asleep::asleep, rooted::rooted};
use crate::process::started::started;
use crate::simulation::fake::{bare::Bare, programs::Programs};

fn ended(status: i32) -> Output {
	Output {
		status: Some(status),
		printed: Vec::new(),
		complained: Vec::new(),
	}
}

#[tokio::test]
async fn a_docker_that_answers_is_asked_once_and_never_again() {
	let (_project, root, _layout) = rooted();
	let programs = Programs::default();
	let kind = asleep(&programs, &root, System::Mac);

	kind.wake().await.unwrap();
	kind.wake().await.unwrap();

	assert_eq!(started(&programs), ["docker info"]);
}

#[tokio::test]
async fn a_stopped_docker_is_started_on_a_mac_and_waited_for() {
	let (_project, root, _layout) = rooted();
	let programs = Programs::default();

	programs.scripted.replace(VecDeque::from([
		ended(1),
		ended(0),
		ended(1),
		ended(0),
	]));

	asleep(&programs, &root, System::Mac).wake().await.unwrap();

	assert_eq!(
		started(&programs),
		[
			"docker info",
			"open -g -a Docker",
			"docker info",
			"docker info"
		]
	);
}

#[tokio::test]
async fn a_stopped_docker_on_linux_stops_with_the_command_to_start_it() {
	let (_project, root, _layout) = rooted();
	let programs = Programs::default();

	programs.scripted.replace(VecDeque::from([ended(1)]));

	let refusal = asleep(&programs, &root, System::Linux)
		.wake()
		.await
		.unwrap_err();

	assert_eq!(
		refusal.to_string(),
		"Docker is installed and not running: start it with `sudo systemctl start docker`, then run again."
	);
	assert_eq!(started(&programs), ["docker info"]);
}

#[tokio::test]
async fn a_docker_that_never_answers_is_a_stop_after_the_bound() {
	let (_project, root, _layout) = rooted();
	let programs = Programs::default();
	let never: VecDeque<Output> = std::iter::once(ended(1))
		.chain(std::iter::once(ended(0)))
		.chain(std::iter::repeat_n(ended(1), 90))
		.collect();

	programs.scripted.replace(never);

	let refusal = asleep(&programs, &root, System::Mac)
		.wake()
		.await
		.unwrap_err();

	assert!(matches!(refusal, Error::Asleep));
	assert_eq!(started(&programs).len(), 92);
}

#[tokio::test]
async fn a_mac_with_a_docker_client_and_nothing_to_start_has_no_runtime() {
	let (_project, root, _layout) = rooted();
	let programs = Programs::default();

	programs
		.scripted
		.replace(VecDeque::from([ended(1), ended(1)]));

	let refusal = asleep(&programs, &root, System::Mac)
		.wake()
		.await
		.unwrap_err();

	assert!(matches!(refusal, Error::RuntimelessMac));
}

#[tokio::test]
async fn a_machine_with_no_docker_says_how_its_system_gets_one() {
	let (_project, root, _layout) = rooted();

	let mac = asleep(&Bare, &root, System::Mac).wake().await.unwrap_err();
	let linux = asleep(&Bare, &root, System::Linux)
		.wake()
		.await
		.unwrap_err();

	assert_eq!(
		mac.to_string(),
		"nsx found no container runtime on this Mac: run nsx init in a terminal and answer yes, and nsx installs one."
	);
	assert_eq!(
		linux.to_string(),
		"nsx found no Docker on this machine: install it with `curl -fsSL https://get.docker.com | sh`, then run again."
	);
}
