//! The resolver nsx plans with
//!
//! A component's image is a name derived from what the plan already knows:
//! the runtime, the component's folder and the draft's revision. The owner
//! decided it on 2026-11-11: the name is not verified during a plan, and the
//! local cluster builds the image when a run brings the runtime up.
//!
//! Definition only: the `Derived` type. Its function is the file in
//! `derived/`.
//!
//! ## Rules
//!
//! - The reference is `<runtime id>/<folder>:r<revision>`, where the folder
//!   is the last segment of the component's path:
//!   `book-a-demo-runtime/signup-api:r1`.
//! - Nothing is built, pulled or looked up.

pub mod resolve;

#[derive(Debug, Clone, Copy, Default)]
pub struct Derived;
