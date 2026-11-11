//! What reading a shape can refuse
//!
//! The errors of this folder: a `runtime.md` whose fence cannot be read, and a
//! record whose text does not match its shape. A decoding error is worded to
//! sit inside the sentence of whoever read the record, which names the file.
//!
//! Definition only: the `Error` type.
//!
//! Spec 0077.

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("nsx will not read runtime.md: it is larger than 1 MiB.")]
	TooLarge,
	#[error("nsx needs exactly one langdrift block in runtime.md.")]
	Blocks,
	#[error("nsx found the langdrift block of runtime.md unclosed.")]
	Unclosed,
	#[error("runtime.md's fence is not version 3; re-plan the runtime.")]
	Version,
	#[error(
		"runtime.md's fence does not read as version 3 ({cause}); re-plan the runtime."
	)]
	Fence { cause: Box<Error> },
	#[error("the record does not match its shape: {cause}")]
	Record { cause: serde_json::Error },
	#[error("the text holds more than one record")]
	Records,
}
