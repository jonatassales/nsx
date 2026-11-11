//! The manifest: which file decides a component, and what it decides

use std::path::{Path, PathBuf};

use nsx::components::{
	error::Error,
	manifest::Manifest,
	role::{API, BROKER, CACHE, WEB},
	stack::{GO, NATS, NEXT, NODE, REDIS},
};
use nsx::contract::{
	component::{ROLES, STACKS},
	resolved::MANIFESTS,
};
use nsx::files::root::Root;
use rstest::rstest;

use crate::components::laid::laid;
use crate::simulation::project::project;

#[test]
fn the_five_manifests_of_the_fixture_give_their_stack_and_role() {
	let project = project();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let paths = [
		"services/signup-api",
		"services/notify-api",
		"platform/nats",
		"platform/redis",
		"apps/signup",
	];

	let decided = paths
		.map(|path| Manifest::find(&root, path).unwrap())
		.map(|manifest| (manifest.file(), manifest.classify()));

	assert_eq!(
		decided,
		[
			("go.mod", (GO, API)),
			("package.json", (NODE, API)),
			("nats.conf", (NATS, BROKER)),
			("redis.conf", (REDIS, CACHE)),
			("next.config.ts", (NEXT, WEB)),
		]
	);
}

#[test]
fn next_config_wins_over_the_package_json_beside_it() {
	let files = [("package.json", "{}"), ("next.config.ts", "")];
	let (_project, root) = laid("apps/site", &files);

	let manifest = Manifest::find(&root, "apps/site").unwrap();

	assert_eq!(manifest, Manifest::NextConfig);
	assert_eq!(manifest.classify(), (NEXT, WEB));
}

#[test]
fn a_package_json_alone_never_decides_next() {
	let files = [("package.json", r#"{"dependencies": {"next": "16.0.0"}}"#)];
	let (_project, root) = laid("apps/site", &files);

	let manifest = Manifest::find(&root, "apps/site").unwrap();

	assert_eq!(manifest.classify(), (NODE, API));
}

#[test]
fn every_stack_role_and_manifest_is_one_the_contract_spells() {
	let manifests = [
		Manifest::NextConfig,
		Manifest::GoMod,
		Manifest::PackageJson,
		Manifest::NatsConf,
		Manifest::RedisConf,
	];

	for manifest in manifests {
		let (stack, role) = manifest.classify();

		assert!(MANIFESTS.contains(&manifest.file()));
		assert!(STACKS.contains(&stack.name));
		assert!(ROLES.contains(&role.name));
	}
}

#[test]
fn a_folder_the_project_does_not_hold_is_a_stop() {
	let (_project, root) = laid("services/api", &[]);

	let refusal = Manifest::find(&root, "services/billing").unwrap_err();

	assert!(matches!(refusal, Error::Missing { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		"nsx found no folder services/billing in the project: the spec names it as a component."
	);
}

#[rstest]
#[case::a_dockerfile("Dockerfile")]
#[case::a_pyproject("pyproject.toml")]
#[case::a_next_config_in_javascript("next.config.js")]
fn a_manifest_nobody_recognises_is_a_stop(#[case] file: &str) {
	let (_project, root) = laid("services/api", &[(file, "")]);

	let refusal = Manifest::find(&root, "services/api").unwrap_err();

	assert!(
		matches!(refusal, Error::Unrecognised { .. }),
		"was {refusal:?}"
	);
	assert!(
		refusal.to_string().contains("no manifest in services/api"),
		"said {refusal}"
	);
}

#[rstest]
#[case::a_path_that_climbs("../elsewhere")]
#[case::a_path_from_the_disk_root("/etc")]
fn a_path_outside_the_project_is_a_stop(#[case] path: &str) {
	let (_project, root) = laid("services/api", &[]);

	let refusal = Manifest::find(&root, path).unwrap_err();

	assert!(matches!(refusal, Error::File(_)), "was {refusal:?}");
}

#[rstest]
#[case::nats_with_a_colon(Manifest::NatsConf, "port: 4333\n", Some(4333))]
#[case::nats_with_an_equals_sign(
	Manifest::NatsConf,
	"port = 4333\n",
	Some(4333)
)]
#[case::nats_with_a_space(Manifest::NatsConf, "port 4333\n", Some(4333))]
#[case::nats_after_a_comment(
	Manifest::NatsConf,
	"# port: 1\nserver_name: n1\nport: 4333 # clients\n",
	Some(4333)
)]
#[case::nats_before_a_slashed_comment(
	Manifest::NatsConf,
	"// port: 1\nport: 4333 // the clients' port\n",
	Some(4333)
)]
#[case::nats_with_a_semicolon(Manifest::NatsConf, "port: 4333;\n", Some(4333))]
#[case::nats_with_a_comma(Manifest::NatsConf, "port: 4333,\n", Some(4333))]
#[case::nats_written_as_json(
	Manifest::NatsConf,
	"\"port\": 4333,\n\"server_name\": \"n1\"\n",
	Some(4333)
)]
#[case::nats_wrapped_in_braces(
	Manifest::NatsConf,
	"{\n  \"port\": 5222,\n  \"cluster\": {\"port\": 6222}\n}\n",
	Some(5222)
)]
#[case::nats_wrapped_in_braces_on_one_line(
	Manifest::NatsConf,
	"{ \"port\": 5222 }\n",
	Some(5222)
)]
#[case::nats_after_a_url_in_a_one_line_block(
	Manifest::NatsConf,
	"cluster { name: c, routes: [nats://a:6222] }\nport: 5222\n",
	Some(5222)
)]
#[case::nats_after_a_quoted_hash_in_a_block(
	Manifest::NatsConf,
	"authorization { token: \"ab#cd\" }\nport: 5222\n",
	Some(5222)
)]
#[case::nats_with_a_block_over_several_lines(
	Manifest::NatsConf,
	"cluster {\n  routes: [\n    nats://a:6222\n  ]\n  port: 6222\n}\nport: 5222\n",
	Some(5222)
)]
#[case::nats_listening_in_capitals(
	Manifest::NatsConf,
	"Listen: 0.0.0.0:5222\n",
	Some(5222)
)]
#[case::nats_listening_on_a_quoted_address(
	Manifest::NatsConf,
	"listen: \"[::]:5222\"\n",
	Some(5222)
)]
#[case::nats_with_a_quoted_value(
	Manifest::NatsConf,
	"port: \"4333\"\n",
	Some(4333)
)]
#[case::nats_listening_on_a_host_and_port(
	Manifest::NatsConf,
	"listen: 0.0.0.0:4333\n",
	Some(4333)
)]
#[case::nats_listening_on_a_port(
	Manifest::NatsConf,
	"listen: 4333\n",
	Some(4333)
)]
#[case::nats_with_a_port_only_inside_a_block(
	Manifest::NatsConf,
	"cluster {\n  port: 6222\n  listen: 0.0.0.0:6222\n}\n",
	None
)]
#[case::nats_with_a_port_after_a_block(
	Manifest::NatsConf,
	"cluster {\n  port: 6222\n}\nport: 4333\n",
	Some(4333)
)]
#[case::nats_with_another_key_that_starts_alike(
	Manifest::NatsConf,
	"ports_file_dir: \"/tmp\"\nhttp_port: 8222\n",
	None
)]
#[case::nats_with_a_port_from_the_environment(
	Manifest::NatsConf,
	"port: $NATS_PORT\n",
	None
)]
#[case::redis(Manifest::RedisConf, "bind 127.0.0.1\nport 6380\n", Some(6380))]
#[case::redis_in_capitals(Manifest::RedisConf, "PORT 6380\n", Some(6380))]
#[case::redis_with_a_tab(Manifest::RedisConf, "port\t6380\n", Some(6380))]
#[case::redis_with_no_listen_key(Manifest::RedisConf, "listen 6380\n", None)]
#[case::redis_after_a_brace_in_a_value(
	Manifest::RedisConf,
	"requirepass \"a{b\"\nport 6380\n",
	Some(6380)
)]
#[case::redis_with_a_comment_after_it(
	Manifest::RedisConf,
	"port 6380 # the clients' port\n",
	Some(6380)
)]
#[case::a_script_that_sets_port(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "PORT=8081 node server.js"}}"#,
	Some(8081)
)]
#[case::a_script_with_a_long_flag(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "serve --port 9000"}}"#,
	Some(9000)
)]
#[case::a_script_with_a_long_flag_and_an_equals_sign(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "serve --port=9001"}}"#,
	Some(9001)
)]
#[case::a_script_with_a_short_flag(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "serve -p 9002"}}"#,
	Some(9002)
)]
#[case::the_start_script_and_no_other(
	Manifest::PackageJson,
	r#"{"scripts": {"dev": "vite --port 5173", "start": "node server.js"}}"#,
	None
)]
#[case::a_quoted_port(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "PORT=\"8081\" node server.js"}}"#,
	Some(8081)
)]
#[case::a_port_before_a_semicolon(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "PORT=8081; node server.js"}}"#,
	Some(8081)
)]
#[case::a_port_set_through_another_command(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "cross-env PORT=8083 node server.js"}}"#,
	Some(8083)
)]
#[case::a_short_flag_with_its_digits_attached(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "next start -p3001"}}"#,
	Some(3001)
)]
#[case::a_short_flag_that_is_another_tools(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "tsc -p tsconfig.json && serve -p 9004"}}"#,
	Some(9004)
)]
#[case::a_port_taken_from_the_environment(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "serve --port $PORT"}}"#,
	None
)]
#[case::a_package_without_scripts(Manifest::PackageJson, "{}", None)]
#[case::a_go_module(Manifest::GoMod, "module example.com/api\n", None)]
#[case::a_next_config(Manifest::NextConfig, "const port = 3001\n", None)]
fn a_manifest_declares_its_port_or_none(
	#[case] manifest: Manifest,
	#[case] written: &str,
	#[case] port: Option<i64>,
) {
	let file = Path::new("services/api").join(manifest.file());

	assert_eq!(manifest.port(&file, written).unwrap(), port);
}

#[rstest]
#[case::a_port_of_zero(Manifest::RedisConf, "port 0\n", "0")]
#[case::a_port_above_the_range(Manifest::NatsConf, "port: 70000\n", "70000")]
#[case::a_port_that_is_a_word(Manifest::NatsConf, "port: clients\n", "clients")]
#[case::a_port_with_no_value(Manifest::NatsConf, "port:\n", "")]
#[case::a_listen_with_no_port(
	Manifest::NatsConf,
	"listen: localhost\n",
	"localhost"
)]
#[case::a_long_flag_with_a_word(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "serve --port clients"}}"#,
	"clients"
)]
#[case::a_long_flag_with_no_value(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "serve --port="}}"#,
	""
)]
#[case::a_port_variable_with_a_word(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "PORT=web node server.js"}}"#,
	"web"
)]
#[case::a_script_port_above_the_range(
	Manifest::PackageJson,
	r#"{"scripts": {"start": "serve -p 70000"}}"#,
	"70000"
)]
fn a_declared_port_nsx_cannot_read_is_a_stop_never_another_port(
	#[case] manifest: Manifest,
	#[case] written: &str,
	#[case] value: &str,
) {
	let file = Path::new("services/api").join(manifest.file());

	let refusal = manifest.port(&file, written).unwrap_err();

	assert!(matches!(refusal, Error::Declared { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		format!(
			"nsx could not read the port {value:?} that services/api/{} declares: a port is a whole number from 1 to 65535.",
			manifest.file()
		)
	);
}

#[test]
fn a_package_json_that_is_not_json_is_refused() {
	let file = Path::new("services/api/package.json");

	let refusal = Manifest::PackageJson.port(file, "{ scripts: ").unwrap_err();

	assert!(matches!(refusal, Error::Manifest { .. }), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("services/api/package.json"),
		"said {refusal}"
	);
}
