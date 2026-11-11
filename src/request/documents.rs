//! The documents the plan read, together
//!
//! The PRD, the design doc and the spec of one ticket, as the judge reads
//! them beside the candidates: the facts every judgment rests on.
//!
//! Definition only: the `Documents` type: the PRD, the design doc and the
//! spec, each borrowed from the plan that read it.
//!
//! Spec 0078.

use crate::request::{design::Design, prd::Prd, spec::Spec};

#[derive(Debug, Clone, Copy)]
pub struct Documents<'plan> {
	pub prd: &'plan Prd,
	pub design: &'plan Design,
	pub spec: &'plan Spec,
}
