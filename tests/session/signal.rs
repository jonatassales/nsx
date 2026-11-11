//! The system's interrupt: watched from the start, and heard by a wait

use std::time::Duration;

use nsx::session::{interrupt::Interrupt, signal::Signal};

#[tokio::test]
async fn an_interrupt_sent_before_anyone_waits_is_still_heard() {
	let signal = Signal::watch().unwrap();
	let process = std::process::id().to_string();
	let sent = std::process::Command::new("kill")
		.args(["-INT", &process])
		.status()
		.unwrap();

	let heard = tokio::time::timeout(Duration::from_secs(5), signal.wait()).await;

	assert!(sent.success());
	assert_eq!(heard, Ok(()));
}
