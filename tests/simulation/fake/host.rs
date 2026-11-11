//! The cluster that does as it is asked and keeps nothing
//!
//! Every Deployment is available at once and every space is gone once
//! closed, as the simulation shows. A test can make one Deployment never
//! available, or make the space cling after it is closed.

mod cluster;

#[derive(Debug, Clone, Copy)]
pub struct Host {
	pub failing: Option<&'static str>,
	pub clinging: bool,
}

pub const HOST: Host = Host {
	failing: None,
	clinging: false,
};
