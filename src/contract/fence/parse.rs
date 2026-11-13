//! Reading the fence out of `runtime.md`
//!
//! One function: it takes the text of a `runtime.md` and gives back the fence,
//! or refuses the text.
//!
//! ## Rules
//!
//! - The text holds exactly one `langdrift` block.
//! - Any version other than 3 is refused with "re-plan the runtime", so an
//!   older draft is never misread.
//! - A field version 3 does not define is refused.
//! - It reads the shape only. Which rules the fence breaks is `validate`'s to
//!   say.

use serde_json::Value;

use crate::contract::{
	decode::decode,
	error::Error,
	fence::{CLOSING, Fence, OPENING, VERSION},
};

const LIMIT: usize = 1 << 20;

impl Fence {
	pub fn parse(markdown: &str) -> Result<Self, Error> {
		if markdown.len() > LIMIT {
			return Err(Error::TooLarge);
		}

		if markdown.matches(OPENING).count() != 1 {
			return Err(Error::Blocks);
		}

		let Some((_heading, after)) = markdown.split_once(OPENING) else {
			return Err(Error::Blocks);
		};

		let Some((block, _rest)) = after.split_once(CLOSING) else {
			return Err(Error::Unclosed);
		};

		// The version is read before the shape, so that an older fence is
		// refused as older rather than misread.
		let probe = decode::<Value>(block).map_err(|cause| Error::Fence {
			cause: Box::new(cause),
		})?;

		if probe.get("version").and_then(Value::as_i64) != Some(VERSION) {
			return Err(Error::Version);
		}

		decode(block).map_err(|cause| Error::Fence {
			cause: Box::new(cause),
		})
	}
}
