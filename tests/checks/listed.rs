//! Checks as the forge lists them, each with where it stands

use nsx::checks::{check::Check, standing::Standing};

pub fn listed(checks: &[(&str, Standing)]) -> Vec<Check> {
	checks
		.iter()
		.map(|(name, standing)| Check {
			name: String::from(*name),
			standing: *standing,
		})
		.collect()
}
