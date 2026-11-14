//! The pinned tools of this machine
//!
//! One function: it gives back kind and kubectl as nsx pins them for the
//! machine nsx was built for, each with its release address and SHA-256.
//!
//! ## Rules
//!
//! - The addresses are the publishers' own: kind's GitHub release and
//!   Kubernetes' release host.
//! - The SHA-256 of each file is the one its publisher lists beside it.
//! - A machine nsx is not built for has no pinned tool.

use crate::local::tool::{KIND_VERSION, KUBECTL_VERSION, Tool};

// kind and kubectl, per machine: the SHA-256 each publisher lists.
const SUMS: [(&str, &str, &str, &str); 4] = [
	(
		"darwin",
		"arm64",
		"0c8c7dbe5e23594a198b786c4bc13dacc101fa6196b0cb0b23a1ca44e61f4b4f",
		"55701af0d51815ee335f240e48acf0959c3c23017ed3fa816e880c4a92bae581",
	),
	(
		"darwin",
		"amd64",
		"5a99f26f57246dc9319dd294803313197a0f34d33c525b3ea8b655db5916ece0",
		"88379aff5c8111de6be0a394adffba2819e44cde269f3ab23cef76de1a48eaa8",
	),
	(
		"linux",
		"arm64",
		"20022bee6cfcd5086cb7234d218e3454e6090022f2a8f55d1fa7fcf42c3867a2",
		"fcc16f63186659d9722014b7e02d5fdd2d4e669cb8c0ab66365d7d3b8ba03509",
	),
	(
		"linux",
		"amd64",
		"aee6151561422756b764a4ae28e7f44cda5af5a9eead3cc9985112b1de8d8e0d",
		"c04487523e7aa7d1493f7808174c899db93d7a9cf57531420cd76b9bd415f8f4",
	),
];

impl Tool {
	pub fn pinned() -> Vec<Self> {
		let system = match std::env::consts::OS {
			"macos" => "darwin",
			other => other,
		};
		let machine = match std::env::consts::ARCH {
			"aarch64" => "arm64",
			"x86_64" => "amd64",
			other => other,
		};
		let pinned = SUMS
			.iter()
			.find(|(os, arch, ..)| *os == system && *arch == machine);

		let Some((os, arch, kind, kubectl)) = pinned else {
			return Vec::new();
		};

		vec![
			Self {
				name: "kind",
				url: format!(
					"https://github.com/kubernetes-sigs/kind/releases/download/{KIND_VERSION}/kind-{os}-{arch}"
				),
				sha256: String::from(*kind),
			},
			Self {
				name: "kubectl",
				url: format!(
					"https://dl.k8s.io/release/{KUBECTL_VERSION}/bin/{os}/{arch}/kubectl"
				),
				sha256: String::from(*kubectl),
			},
		]
	}
}
