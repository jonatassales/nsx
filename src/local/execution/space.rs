//! The name of a run's space
//!
//! One function: it takes the project's root and a run's id, and gives back
//! the name of the run's space in the cluster.
//!
//! ## Rules
//!
//! - The name is the run's id, cut to fifty characters, then eight
//!   hexadecimal characters of a digest of the project's folder and the
//!   run's id. The cluster is shared by every project on the machine, and
//!   two checkouts of one repository give their runs the same ids: the
//!   digest keeps their spaces apart.
//! - The name is a lowercase label of at most 63 characters, as the cluster
//!   asks of a namespace.
//!
//! Spec 0081.

use sha2::{Digest, Sha256};

use crate::files::root::Root;
use crate::local::execution::Execution;

const READABLE: usize = 50;

const DIGITS: usize = 8;

impl Execution {
	pub fn space(root: &Root, run: &str) -> String {
		let mut digest = Sha256::new();

		digest.update(root.path.as_os_str().as_encoded_bytes());
		digest.update([0]);
		digest.update(run.as_bytes());

		let mark = digest
			.finalize()
			.iter()
			.map(|byte| format!("{byte:02x}"))
			.collect::<String>();
		let readable = run.chars().take(READABLE).collect::<String>();

		format!("{}-{}", readable.trim_end_matches('-'), &mark[..DIGITS])
	}
}
