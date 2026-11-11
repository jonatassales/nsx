//! The image a component runs
//!
//! Each component has one image reference, resolved by nsx during the plan.
//! The plan records it in the `runtime.md` draft, and the local cluster later
//! runs it (spec 0062). The plan resolves images. It never builds or runs one.
//!
//! Definition only: the `Image` type: one resolved image reference, as text.
//! Its functions are the files in `image/`. What resolves it is the
//! `Resolver`, in its own file.
//!
//! ## Rules
//!
//! - The image is resolved by nsx. It is never typed into the fence and never
//!   sent by the console (what survives of ADR 0004).
//! - The recorded image is the resolved one. The derived tag of ADR 0004 is
//!   gone.
//! - The spec puts the resolver behind a port, so that the golden test can use
//!   one that resolves all five components.
//!
//! ## Open
//!
//! - Whether a plan should ever verify that the image exists. The owner's
//!   answer of 2026-11-11 is a derived name, not verified (`derived.rs`).
//!
//! Spec 0079.

pub mod resolve;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image(pub String);
