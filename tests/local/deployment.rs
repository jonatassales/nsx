//! One Deployment per component, at exactly its share

use nsx::local::{deployment::Deployment, error::Error};
use rstest::rstest;

use crate::local::component::component;

#[rstest]
#[case("services/signup-api", "services-signup-api")]
#[case("platform/nats", "platform-nats")]
#[case("apps/signup", "apps-signup")]
#[case("web", "web")]
fn a_deployment_is_named_after_its_components_path(
	#[case] path: &str,
	#[case] name: &str,
) {
	let deployment = Deployment::of(&component(path)).unwrap();

	assert_eq!(deployment.name, name);
	assert_eq!(deployment.image, "book-a-demo-runtime/signup-api:r1");
	assert_eq!(
		(
			deployment.port,
			deployment.cpu_millis,
			deployment.memory_mib
		),
		(8081, 250, 256)
	);
}

#[rstest]
#[case::capitals("apps/Signup")]
#[case::an_underscore("apps/sign_up")]
#[case::a_dot("apps/signup.web")]
#[case::a_trailing_slash("apps/signup/")]
fn a_path_that_makes_no_label_names_no_deployment(#[case] path: &str) {
	let refusal = Deployment::of(&component(path)).unwrap_err();

	assert!(matches!(refusal, Error::Unnamed { .. }), "was {refusal:?}");
	assert!(refusal.to_string().contains(path), "said {refusal}");
}

#[test]
fn the_manifest_requests_and_limits_exactly_the_share() {
	let deployment = Deployment::of(&component("services/signup-api")).unwrap();

	let manifest = deployment.manifest("notes-0001");
	let container = &manifest["spec"]["template"]["spec"]["containers"][0];

	assert_eq!(manifest["kind"], "Deployment");
	assert_eq!(manifest["metadata"]["name"], "services-signup-api");
	assert_eq!(manifest["metadata"]["namespace"], "notes-0001");
	assert_eq!(
		manifest["metadata"]["labels"]["langdrift.md/space"],
		"notes-0001"
	);
	assert_eq!(manifest["spec"]["replicas"], 1);
	assert_eq!(manifest["spec"]["progressDeadlineSeconds"], 1200);
	assert_eq!(container["image"], "book-a-demo-runtime/signup-api:r1");
	assert_eq!(container["ports"][0]["containerPort"], 8081);
	assert_eq!(container["resources"]["requests"]["cpu"], "250m");
	assert_eq!(container["resources"]["requests"]["memory"], "256Mi");
	assert_eq!(
		container["resources"]["limits"],
		container["resources"]["requests"]
	);
}
