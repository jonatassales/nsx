//! What gh says of a pull request's head commit

use nsx::process::output::Output;

use crate::process::said::said;

pub fn headed(commit: &str) -> Output {
	said(0, &format!(r#"{{"headRefOid": "{commit}"}}"#))
}
