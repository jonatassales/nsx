//! Checking the fence against its rules
//!
//! One function: it takes a fence and gives back every problem it finds, as
//! one list. An empty list is a valid fence. The length of the list is the
//! count the plan's `validate` line prints.
//!
//! ## Rules
//!
//! - The bounds are the ones Core's contract enforced, in its Go package
//!   `configuration` (removed with ticket 0434); spec 0049 names the fields.
//! - It collects every problem and never stops at the first.
//! - `version` is 3, `runtimeId` is a lowercase DNS label and `revision` is at
//!   least 1.
//! - The bound is 1 to 64,000 millicores and 1 to 1,048,576 MiB.
//! - `model` is not empty and `effort` is one of the three. Whether the model
//!   is in the price list is spec 0056's check.
//! - There are 1 to 16 components.
//! - Each `path` is clean, relative, outside `.langdrift` and unique.
//! - Each `stack` and each `role` is in its closed set.
//! - Each `port` is 1 to 65535 and distinct.
//! - Each `image` is an image reference and distinct.
//! - Each share is positive, and the shares sum to at most the bound.
//! - Each `replicas` is 1.
//!
//! Spec 0077.

use std::collections::HashSet;

use crate::contract::{
	component::{PORTS, ROLES, STACKS},
	fence::{
		EFFORTS, Fence, MAXIMUM_COMPONENTS, MAXIMUM_CPU_MILLIS, MAXIMUM_MEMORY_MIB,
		VERSION,
	},
	label::label,
	plain::plain,
	problem::Problem,
	reference::reference,
};

impl Fence {
	pub fn validate(&self) -> Vec<Problem> {
		// A slash-separated path that is already clean and never climbs.
		fn is_clean(text: &str) -> bool {
			let lexical =
				plain(text) && !text.contains('\\') && !text.starts_with('/');

			lexical
				&& text
					.split('/')
					.all(|segment| !matches!(segment, "" | "." | ".."))
		}

		let mut problems = Vec::new();

		if self.version != VERSION {
			problems.push(Problem::at("version", "must be 3; re-plan the runtime"));
		}

		if !label(&self.runtime_id) {
			problems.push(Problem::at("runtimeId", "must be a lowercase DNS label"));
		}

		if self.revision < 1 {
			problems.push(Problem::at("revision", "must be at least 1"));
		}

		if !(1..=MAXIMUM_CPU_MILLIS).contains(&self.cpu_millis) {
			let rule = format!("must be 1 to {MAXIMUM_CPU_MILLIS} millicores");

			problems.push(Problem::at("cpuMillis", &rule));
		}

		if !(1..=MAXIMUM_MEMORY_MIB).contains(&self.memory_mib) {
			let rule = format!("must be 1 to {MAXIMUM_MEMORY_MIB} MiB");

			problems.push(Problem::at("memoryMiB", &rule));
		}

		if self.model.is_empty() {
			problems.push(Problem::at("model", "must name a model"));
		}

		if !EFFORTS.contains(&self.effort.as_str()) {
			let rule = format!("must be one of {}", EFFORTS.join(", "));

			problems.push(Problem::at("effort", &rule));
		}

		if !(1..=MAXIMUM_COMPONENTS).contains(&self.components.len()) {
			let rule = format!("must hold 1 to {MAXIMUM_COMPONENTS} components");

			problems.push(Problem::at("components", &rule));
		}

		for (index, component) in self.components.iter().enumerate() {
			let field = |name: &str| format!("components[{index}].{name}");
			let inside = component.path == ".langdrift"
				|| component.path.starts_with(".langdrift/");

			if !is_clean(&component.path) {
				problems
					.push(Problem::at(&field("path"), "must be a clean relative path"));
			} else if inside {
				problems
					.push(Problem::at(&field("path"), "must be outside .langdrift"));
			}

			if !STACKS.contains(&component.stack.as_str()) {
				let rule = format!("must be one of {}", STACKS.join(", "));

				problems.push(Problem::at(&field("stack"), &rule));
			}

			if !ROLES.contains(&component.role.as_str()) {
				let rule = format!("must be one of {}", ROLES.join(", "));

				problems.push(Problem::at(&field("role"), &rule));
			}

			if !PORTS.contains(&component.port) {
				problems.push(Problem::at(&field("port"), "must be 1 to 65535"));
			}

			if !reference(&component.image) {
				problems
					.push(Problem::at(&field("image"), "must be an image reference"));
			}

			if component.cpu_millis < 1 {
				problems
					.push(Problem::at(&field("cpuMillis"), "must be a positive share"));
			}

			if component.memory_mib < 1 {
				problems
					.push(Problem::at(&field("memoryMiB"), "must be a positive share"));
			}

			if component.replicas != 1 {
				problems.push(Problem::at(&field("replicas"), "must be 1"));
			}
		}

		// Paths, ports and images are each unique across the runtime, so a
		// later repeat is the problem.
		let mut paths = HashSet::new();
		let mut ports = HashSet::new();
		let mut images = HashSet::new();

		for (index, component) in self.components.iter().enumerate() {
			let field = |name: &str| format!("components[{index}].{name}");

			if !paths.insert(component.path.as_str()) {
				problems.push(Problem::at(&field("path"), "must be unique"));
			}

			if !ports.insert(component.port) {
				problems.push(Problem::at(&field("port"), "must be distinct"));
			}

			if !images.insert(component.image.as_str()) {
				problems.push(Problem::at(&field("image"), "must be distinct"));
			}
		}

		// A draft never proposes more than the design doc allows.
		let cpu_millis = self.components.iter().fold(0, |sum: i64, component| {
			sum.saturating_add(component.cpu_millis)
		});
		let memory_mib = self.components.iter().fold(0, |sum: i64, component| {
			sum.saturating_add(component.memory_mib)
		});

		if cpu_millis > self.cpu_millis {
			problems.push(Problem::at(
				"components.cpuMillis",
				"the shares exceed the bound",
			));
		}

		if memory_mib > self.memory_mib {
			problems.push(Problem::at(
				"components.memoryMiB",
				"the shares exceed the bound",
			));
		}

		problems
	}
}
