//! The fakes: what stands in for the terminal and the outside systems
//!
//! One per trait. Nothing here is under `src/`: only the tests use it.

pub mod answering;
pub mod answers;
pub mod calm;
pub mod coder;
pub mod console;
pub mod forged;
pub mod hasty;
pub mod host;
pub mod hub;
pub mod judge;
pub mod machine;
pub mod moment;
pub mod offer;
pub mod programs;
pub mod recorded;
pub mod recorder;
pub mod refuser;
pub mod served;
pub mod session;
pub mod silence;
pub mod spender;
pub mod witness;
