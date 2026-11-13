//! The Deployment as the cluster reads it
//!
//! One function: it takes a Deployment and the run's space, and gives back
//! the manifest the cluster applies.
//!
//! ## Rules
//!
//! - One container runs the resolved image on the component's port. An image
//!   already on the node is used as it is, and any other is pulled.
//! - Cpu and memory are requested and limited at the same figures.
//! - The Deployment's progress deadline is the bound nsx waits for it, so
//!   that the cluster does not give up on it sooner.
//! - The Deployment carries the run's label, as its space does.

use serde_json::{Value, json};

use crate::local::{
	deployment::Deployment,
	kind::{LABEL, WAIT},
};

impl Deployment {
	pub fn manifest(&self, space: &str) -> Value {
		let share = json!({
			"cpu": format!("{}m", self.cpu_millis),
			"memory": format!("{}Mi", self.memory_mib),
		});
		let chosen = json!({
			"app": self.name,
		});

		json!({
			"apiVersion": "apps/v1",
			"kind": "Deployment",
			"metadata": {
				"name": self.name,
				"namespace": space,
				"labels": {
					LABEL: space,
				},
			},
			"spec": {
				"replicas": self.replicas,
				"progressDeadlineSeconds": WAIT,
				"selector": {
					"matchLabels": chosen,
				},
				"template": {
					"metadata": {
						"labels": chosen,
					},
					"spec": {
						"containers": [{
							"name": "component",
							"image": self.image,
							"imagePullPolicy": "IfNotPresent",
							"ports": [{
								"containerPort": self.port,
							}],
							"resources": {
								"requests": share,
								"limits": share,
							},
						}],
					},
				},
			},
		})
	}
}
