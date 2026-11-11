//! The `runtime.md` draft
//!
//! The draft is the runtime's file as the plan proposes it. It holds the
//! `langdrift` fence, whose fields are spec 0049's, and beside it the sizing
//! report. It keeps only what the plan shows and what the run applies.
//!
//! Definition only: the `Draft` type: the fence and the sizing the report
//! is written from. Its functions are the files in `draft/`.
//!
//! ## Rules
//!
//! - The fence carries the runtime's id and revision, the bound, the judge's
//!   suggested model and effort, and per component its path, stack, role,
//!   port, image, proposed share and one replica, in the spec's order.
//! - The runtime-wide cpu and memory are the bound. The per-component cpu and
//!   memory are the proposed shares.
//! - The model and effort are the judge's suggestion, recorded as a
//!   recommendation, and stay so. The team's applied choice lives in the
//!   recorded intent, and the run never rewrites `runtime.md`.
//! - The sizing report carries the proposed totals, the bound, the headroom,
//!   the resource, the limits statement and the validation result.
//! - Amounts are whole millicores and whole MiB.
//!
//! Spec 0079.

pub mod card;
pub mod line;
pub mod of;
pub mod validate;
pub mod write;

use crate::contract::fence::Fence;
use crate::runtime::sizing::Sizing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Draft {
	pub fence: Fence,
	pub sizing: Sizing,
}
