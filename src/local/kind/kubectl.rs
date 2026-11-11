//! A kubectl call on the shared cluster
//!
//! One function: it takes kubectl's arguments and what it reads, and gives
//! back the program, aimed at the shared cluster through the project's own
//! kubeconfig.
//!
//! ## Rules
//!
//! - Every call names the kubeconfig and the context: kubectl never acts on
//!   whatever cluster the person's own configuration points at.
//! - They follow the arguments, so that the call's first argument is its
//!   verb, which a failure names.
//!
//! Spec 0081.

use crate::local::{
	error::Error,
	kind::{CONTEXT, Kind, PASSED},
};
use crate::process::program::Program;

impl Kind<'_> {
	pub fn kubectl(
		&self,
		arguments: &[&str],
		input: Vec<u8>,
	) -> Result<Program, Error> {
		let kubeconfig = self.kubeconfig()?;
		let aimed = [
			arguments,
			&["--kubeconfig", &kubeconfig, "--context", CONTEXT],
		]
		.concat();

		Ok(Program {
			input,
			..Program::tool("kubectl", &aimed, &PASSED, &self.directory)
		})
	}
}
