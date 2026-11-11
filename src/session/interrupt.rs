//! The person's interrupt, as a trait
//!
//! A person stops a command with Ctrl-C, and the system stops one by ending
//! it or by closing its terminal. A run watches for it itself, because it
//! has a runtime to take down and records to close before it ends.
//!
//! Definition only: the trait `Interrupt`, which waits until the command is
//! interrupted, so that tests can replace it with a fake.
//!
//! ## Rules
//!
//! - An interrupt is a stop (spec 0048): one sentence on standard error.
//!
//! Spec 0077.

use std::future::Future;
use std::pin::Pin;

pub trait Interrupt {
	fn wait(&self) -> Pin<Box<dyn Future<Output = ()> + '_>>;
}
