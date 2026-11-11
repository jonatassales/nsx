//! A program sees only the variables it is given

use nsx::process::environment::Environment;

#[test]
fn the_base_carries_the_path() {
	let environment = Environment::allow(&[]);

	assert!(environment.variables.contains_key("PATH"));
}

#[test]
fn a_named_variable_passes() {
	let environment = Environment::allow(&["CARGO_PKG_NAME"]);

	assert_eq!(
		environment
			.variables
			.get("CARGO_PKG_NAME")
			.map(String::as_str),
		Some("nsx")
	);
}

#[test]
fn a_variable_nobody_names_does_not_pass() {
	let environment = Environment::allow(&[]);

	assert!(std::env::var("CARGO_PKG_NAME").is_ok());
	assert!(!environment.variables.contains_key("CARGO_PKG_NAME"));
}

#[test]
fn a_named_variable_that_is_not_set_is_left_out() {
	let environment = Environment::allow(&["NSX_NEVER_SET_ANYWHERE"]);

	assert!(!environment.variables.contains_key("NSX_NEVER_SET_ANYWHERE"));
}
