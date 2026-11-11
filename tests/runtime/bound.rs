//! The simulation's bound: 2000m and 3072 MiB, set in the design doc

use std::path::PathBuf;

use nsx::harness::envelope::Envelope;

pub fn bound() -> Envelope {
	Envelope {
		cpu_millis: 2000,
		memory_mib: 3072,
		source: PathBuf::from(".agents/0005-DD-book-a-demo-runtime.dd.md"),
	}
}
