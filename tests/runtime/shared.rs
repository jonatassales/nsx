//! A resolved component with a share of its own

use nsx::components::component::Component;

use crate::components::resolved::resolved;

pub fn shared(path: &str, cpu_millis: i64, memory_mib: i64) -> Component {
	Component {
		cpu_millis,
		memory_mib,
		..resolved(path, 8080, None)
	}
}
