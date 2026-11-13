//! A component's inspect line
//!
//! One function: it gives back the line the plan prints when it inspects a
//! component: the path, the manifest it read and the port. The golden has
//! five, one per component of the simulation's spec.
//!
//! ## Simulation
//!
//! - `inspectRust`: `console › tool call: inspect services/signup-api · Cargo.toml · :8080`
//! - `inspectNode`: `console › tool call: inspect services/notify-api · package.json · :8081`
//! - `inspectNats`: `console › tool call: inspect platform/nats · nats.conf · :4222`
//! - `inspectRedis`: `console › tool call: inspect platform/redis · redis.conf · :6379`
//! - `inspectNext`: `console › tool call: inspect apps/signup · next.config.ts · :3000`
//!
//! ## Rules
//!
//! - One line per component, in the spec's order, each before its result row.
//! - A component that cannot be inspected prints no line: the plan stops
//!   before it.

use crate::components::component::Component;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Component {
	pub fn line(&self) -> Line {
		Line {
			id: String::from(self.stack.inspect),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!(
					"tool call: inspect {} · {} · :{}",
					self.path,
					self.manifest.file(),
					self.port
				),
			},
		}
	}
}
