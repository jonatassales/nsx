//! A driver configuration with every kind of argument

use nsx::driver::{config::Config, paths::Paths};

pub fn configured() -> Config {
	Config {
		executable: String::from("coding-agent"),
		args: vec![
			String::from("--model={{model}}"),
			String::from("--effort"),
			String::from("{{effort}}"),
			String::from("{{stage}} of {{ticket}} in {{run}}"),
			String::from("{{prompt}}"),
		],
		pass_env: vec![String::from("NSX_TEST_NEVER_SET")],
		usage: Paths {
			input_tokens: String::from("usage.input_tokens"),
			output_tokens: String::from("usage.output_tokens"),
		},
	}
}
