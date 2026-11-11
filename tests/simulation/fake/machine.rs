//! The cluster that keeps what is created in it
//!
//! Each space holds the Deployments applied in it, and every operation is
//! kept in the order it was asked. A test can make one Deployment never
//! available, which is created all the same, as a real one would be; make
//! one Deployment refuse to be deleted; make a closed space stay; make the
//! cluster refuse everything, as one that cannot be reached; or make it
//! never answer when a space is closed.

mod cluster;
mod reach;
mod refusal;

use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Machine {
	pub spaces: RefCell<BTreeMap<String, Vec<String>>>,
	pub asked: RefCell<Vec<String>>,
	pub failing: Option<&'static str>,
	pub stuck: Option<&'static str>,
	pub clinging: bool,
	pub unreachable: bool,
	pub stalling: bool,
}
