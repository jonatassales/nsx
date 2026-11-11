//! The client's components
//!
//! The plan resolves the components the spec names, in the spec's order. For
//! each path it finds the manifest, which gives the stack and the role, takes
//! the port, resolves the image and proposes a share. Then it cross-checks the
//! components for collisions. Each resolved component goes to the `runtime.md`
//! draft (spec 0055) and to the plan's card data (spec 0050).
//!
//! - `Component`: one component and what the plan resolved for it
//! - `Manifest`: the file that says what a component is
//! - `Stack`: the five stacks
//! - `Role`: the four roles
//! - `Image`: the image a component runs
//! - `Resolver`: what resolves an image, and `Derived`, the one nsx plans
//!   with: a name derived from the runtime, the folder and the revision
//! - `Inventory`: the components the spec names
//! - `Collisions`: the cross-check of ports, images and paths
//!
//! ## Rules
//!
//! - What survives of ADR 0004: a component is its folder in the client's
//!   project, named by its path; its image is resolved by nsx, never typed
//!   into the fence or sent by the console; components are unique by path.
//! - What ADR 0005 superseded: the Dockerfile as the component's definition,
//!   the derived image tag, `EXPOSE` as the port, `HEALTHCHECK` as readiness,
//!   the env file and its Secret.
//! - No Dockerfile, healthcheck, env file or ceiling takes part in the plan.
//! - Nothing is built, loaded, started or probed here: images are resolved,
//!   never built or run (spec 0062).
//! - Reading the spec and counting its components belong to spec 0053.
//!
//! Spec 0079.

pub mod collisions;
pub mod component;
pub mod derived;
pub mod error;
pub mod image;
pub mod inventory;
pub mod manifest;
pub mod resolver;
pub mod role;
pub mod stack;
