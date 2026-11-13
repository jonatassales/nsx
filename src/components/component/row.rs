//! A component's result row
//!
//! One function: it gives back the success line the plan prints once a
//! component is resolved: its path, its stack and role, and its proposed
//! share. The golden has five, one per component of the simulation's spec.
//!
//! ## Simulation
//!
//! - `rust`: `✓ services/signup-api   Rust · api      250m · 256 MiB`
//! - `node`: `✓ services/notify-api   Node.js · api   300m · 512 MiB`
//! - `nats`: `✓ platform/nats         NATS · broker   200m · 256 MiB`
//! - `redis`: `✓ platform/redis        Redis · cache   200m · 256 MiB`
//! - `next`: `✓ apps/signup           Next.js · web   500m · 1024 MiB`
//! - Card: Plan · COMPONENTS rows, each set by its own row line
//!
//! ## Rules
//!
//! - The values are the path, `<stack> · <role>` and `<cpu>m · <memory> MiB`.
//!   The padding between the columns belongs to `session`.
//! - The card's row carries the same data: the stack's mark, the path, the
//!   stack, the role, the cpu and the memory, with a bar of the component's
//!   cpu as a share of the bound's cpu.
//! - Numbers print as plain integers: `1024 MiB`, not `1,024 MiB`. The
//!   grouping is the card's.

use crate::components::component::Component;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Component {
	pub fn row(&self) -> Line {
		Line {
			id: String::from(self.stack.row),
			kind: Kind::Success,
			shape: Shape::Component {
				path: self.path.clone(),
				stack_role: format!("{} · {}", self.stack.name, self.role.name),
				share: format!("{}m · {} MiB", self.cpu_millis, self.memory_mib),
			},
		}
	}
}
