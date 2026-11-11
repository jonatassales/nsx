//! kind, kubectl and Docker, as the cluster the run asks

use nsx::files::root::Root;
use nsx::local::cluster::Cluster;

use crate::local::{deployed::deployed, reached::reached, rooted::rooted};
use crate::process::{said::said, started::started};
use crate::simulation::fake::programs::Programs;

const KUBECONFIG: &str = ".langdrift/agent/state/cluster/kubeconfig";

#[tokio::test]
async fn a_machine_with_no_cluster_gets_one_and_the_run_its_namespace() {
	let (project, root, _layout) = rooted();
	let programs = Programs::default();
	let kubeconfig = project.path().join(KUBECONFIG);
	let file = kubeconfig.display();

	reached(&programs, &root).open("notes-0001").await.unwrap();

	let namespace = serde_json::from_slice::<serde_json::Value>(
		&programs.started.borrow()[2].input,
	)
	.unwrap();

	assert_eq!(
		started(&programs),
		[
			String::from("kind get clusters"),
			format!(
				"kind create cluster --name langdrift --kubeconfig {file} --wait 120s"
			),
			format!(
				"kubectl apply -f - --kubeconfig {file} --context kind-langdrift"
			),
		]
	);
	assert_eq!(namespace["kind"], "Namespace");
	assert_eq!(namespace["metadata"]["name"], "notes-0001");
	assert_eq!(
		namespace["metadata"]["labels"]["langdrift.md/space"],
		"notes-0001"
	);
}

#[tokio::test]
async fn the_shared_cluster_is_reached_through_the_projects_own_kubeconfig() {
	let (project, root, _layout) = rooted();
	let programs = Programs::default();
	let kubeconfig = project.path().join(KUBECONFIG);
	let file = kubeconfig.display();

	programs
		.scripted
		.borrow_mut()
		.push_back(said(0, "kind\nlangdrift\n"));

	reached(&programs, &root).open("notes-0001").await.unwrap();

	assert_eq!(
		started(&programs),
		[
			String::from("kind get clusters"),
			format!("kind export kubeconfig --name langdrift --kubeconfig {file}"),
			format!(
				"kubectl apply -f - --kubeconfig {file} --context kind-langdrift"
			),
		]
	);
}

#[tokio::test]
async fn an_image_on_this_machine_is_loaded_before_the_deployment_is_applied() {
	let (project, root, _layout) = rooted();
	let programs = Programs::default();
	let kubeconfig = project.path().join(KUBECONFIG);
	let file = kubeconfig.display();

	reached(&programs, &root)
		.apply("notes-0001", &deployed("api"))
		.await
		.unwrap();

	let applied = String::from_utf8(programs.started.borrow()[2].input.clone());

	assert_eq!(
		started(&programs),
		[
			String::from("docker image inspect -- notes/api:r1"),
			String::from("kind load docker-image --name langdrift -- notes/api:r1"),
			format!(
				"kubectl apply -f - --kubeconfig {file} --context kind-langdrift"
			),
			format!(
				"kubectl rollout status deployment/api --namespace notes-0001 --timeout 1200s --kubeconfig {file} --context kind-langdrift"
			),
		]
	);
	assert!(applied.unwrap().contains(r#""image":"notes/api:r1""#));
}

#[tokio::test]
async fn an_image_docker_does_not_hold_is_left_for_the_cluster_to_pull() {
	let (_project, root, _layout) = rooted();
	let programs = Programs::default();

	programs.scripted.borrow_mut().push_back(said(1, ""));

	reached(&programs, &root)
		.apply("notes-0001", &deployed("api"))
		.await
		.unwrap();

	assert_eq!(started(&programs).len(), 3);
	assert!(!started(&programs).iter().any(|tool| tool.contains("load")));
}

#[tokio::test]
async fn a_deployment_that_is_never_available_is_a_stop_that_names_the_tool() {
	let (_project, root, _layout) = rooted();
	let programs = Programs::default();

	programs.scripted.borrow_mut().extend([
		said(1, ""),
		said(0, ""),
		said(1, "error: timed out waiting for the condition"),
	]);

	let refusal = reached(&programs, &root)
		.apply("notes-0001", &deployed("api"))
		.await
		.unwrap_err();

	assert_eq!(
		refusal.to_string(),
		"kubectl rollout ended with the exit status 1."
	);
}

#[tokio::test]
async fn a_cluster_that_does_not_exist_holds_nothing_to_remove() {
	let (_project, root, _layout) = rooted();
	let programs = Programs::default();
	let kind = reached(&programs, &root);
	let cluster: &dyn Cluster = &kind;

	cluster.delete("notes-0001", "api").await.unwrap();
	cluster.close("notes-0001").await.unwrap();

	let absent = cluster.absent("notes-0001").await.unwrap();

	assert!(absent);
	assert_eq!(started(&programs), ["kind get clusters"; 3]);
}

#[tokio::test]
async fn what_the_run_created_is_removed_by_its_name_and_its_label() {
	let (project, root, _layout) = rooted();
	let programs = Programs {
		printed: String::from("langdrift\n"),
		..Programs::default()
	};
	let kubeconfig = project.path().join(KUBECONFIG);
	let file = kubeconfig.display();
	let cluster = reached(&programs, &root);

	cluster.delete("notes-0001", "api").await.unwrap();
	cluster.close("notes-0001").await.unwrap();

	let tools = started(&programs);

	// The cluster is reached once: neither kind call is made again.
	assert_eq!(tools.len(), 4);
	assert_eq!(tools[0], "kind get clusters");
	assert!(
		tools[1].starts_with("kind export kubeconfig"),
		"{}",
		tools[1]
	);
	assert_eq!(
		tools[2],
		format!(
			"kubectl delete deployment api --namespace notes-0001 --ignore-not-found --wait=true --kubeconfig {file} --context kind-langdrift"
		)
	);
	assert_eq!(
		tools[3],
		format!(
			"kubectl delete namespace --selector langdrift.md/space=notes-0001 --ignore-not-found --wait=true --kubeconfig {file} --context kind-langdrift"
		)
	);
}

#[tokio::test]
async fn a_namespace_the_cluster_still_lists_is_not_absent() {
	let (_project, root, _layout) = rooted();
	let gone = Programs::default();
	let held = Programs::default();

	gone.scripted.borrow_mut().extend([
		said(0, "langdrift\n"),
		said(0, ""),
		said(0, "\n"),
	]);
	held.scripted.borrow_mut().extend([
		said(0, "langdrift\n"),
		said(0, ""),
		said(0, "namespace/notes-0001\n"),
	]);

	assert!(reached(&gone, &root).absent("notes-0001").await.unwrap());
	assert!(!reached(&held, &root).absent("notes-0001").await.unwrap());
}

#[tokio::test]
async fn outside_a_project_there_is_no_kubeconfig_to_reach_the_cluster_with() {
	let outside = tempfile::tempdir().unwrap();
	let programs = Programs::default();
	let nowhere = Root {
		path: outside.path().to_path_buf(),
	};

	reached(&programs, &nowhere)
		.open("notes-0001")
		.await
		.unwrap_err();

	assert!(
		!started(&programs)
			.iter()
			.any(|tool| tool.starts_with("kubectl"))
	);
}

#[tokio::test]
async fn the_tools_see_no_variable_nsx_did_not_name() {
	let (project, root, _layout) = rooted();
	let programs = Programs::default();

	reached(&programs, &root).open("notes-0001").await.unwrap();

	let started = programs.started.borrow();
	let named = [
		"PATH",
		"HOME",
		"TMPDIR",
		"LANG",
		"LC_ALL",
		"LC_CTYPE",
		"LC_MESSAGES",
		"DOCKER_HOST",
		"XDG_CONFIG_HOME",
		"XDG_RUNTIME_DIR",
	];
	let unnamed = started
		.iter()
		.flat_map(|program| program.environment.variables.keys())
		.find(|name| !named.contains(&name.as_str()));

	assert_eq!(unnamed, None);
	assert_eq!(started[0].directory, project.path());
}
