//! The runtime's two lines, and its section of the run's record

use nsx::local::state::State;
use nsx::session::kind::Kind;

use crate::evidence::recorded::recorded;
use crate::local::{applied::applied, deployed::deployed};

#[test]
fn a_ready_runtime_says_its_revision_its_deployments_and_their_sum() {
	let deployments = [deployed("a"), deployed("b"), deployed("c")];
	let mut run = recorded();

	let state = State::of(&applied(), &deployments);
	let line = state.line();

	state.card(&mut run);

	let section = run.runtime.unwrap();

	assert_eq!(
		(line.id.as_str(), line.kind),
		("runtimeReady", Kind::Output)
	);
	assert_eq!(
		line.render(),
		"runtime    book-a-demo-runtime · revision 1 · 3 Deployments ready · 750m · 768 MiB"
	);
	assert_eq!(
		(
			section.revision,
			section.deployments,
			section.state.as_str()
		),
		(1, 3, "ready")
	);
	assert_eq!((section.cpu_millis, section.memory_mib), (750, 768));
	assert_eq!((section.removed, section.left), (None, None));
}

#[test]
fn a_stopped_runtime_says_what_was_removed_and_that_nothing_is_left() {
	let state = State {
		removed: Some(5),
		..State::of(&applied(), &[deployed("a")])
	};

	let line = state.line();

	assert_eq!(
		(line.id.as_str(), line.kind),
		("runtimeStopped", Kind::Output)
	);
	assert_eq!(
		line.render(),
		"runtime    stopped · 5 Deployments removed · nothing left"
	);
}
