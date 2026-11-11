//! The golden test: nsx plays the simulation
//!
//! The plan and the five runs play against a copy of the fixture project, and
//! what they print is compared with the goldens, line by line. A line not
//! built yet is on the pending list, which only shrinks.

pub mod authorized;
mod budget;
pub mod diff;
pub mod fake;
pub mod fixture;
pub mod frozen;
pub mod golden;
mod haiku;
pub mod indexed;
mod merge;
pub mod pending;
mod plan;
pub mod printed;
pub mod project;
mod ran;
mod rules;
mod run;
mod sketch;
mod stages;
