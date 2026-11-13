//! The interrupt the system sends
//!
//! Definition only: the `Signal` type, which is `Interrupt` as the system
//! sends it: Ctrl-C, a request to end, or the terminal closing. It holds
//! the three signals it watches from the moment it is made. Its functions
//! are the files in `signal/`.
//!
//! ## Rules
//!
//! - The signals are watched from the start of the command, so that one
//!   that arrives before anyone waits is not lost, and does not end nsx
//!   without its sentence.
//! - nsx runs on macOS and Linux: the signals are those systems'.

pub mod wait;
pub mod watch;

use tokio::signal::unix::Signal as Stream;
use tokio::sync::Mutex;

pub struct Signal {
	pub streams: Mutex<[Stream; 3]>,
}
