//! The forecast is read as `nsx init` laid it down

use std::path::PathBuf;

use nsx::command::init::Init;
use nsx::files::{layout::Layout, root::Root};
use nsx::money::{error::Error, forecast::Forecast, tokens::Tokens};
use rstest::rstest;

use crate::simulation::fake::recorder::Recorder;
use crate::simulation::fake::session::session;

#[tokio::test]
async fn the_forecast_reads_as_init_laid_it_down() {
	let project = tempfile::tempdir().unwrap();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let medium = Tokens {
		input: 3500,
		output: 250,
	};

	Init.execute(&mut session).await.unwrap();

	let forecast = Forecast::load(&root, &Layout::under(&root)).unwrap();

	assert_eq!(forecast.unit, "thousands-of-tokens");
	assert_eq!(forecast.by_effort.len(), 3);
	assert_eq!(forecast.by_effort["medium"], medium);
}

#[test]
fn a_project_without_a_forecast_is_a_stop() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Forecast::load(&root, &Layout::under(&root)).unwrap_err();

	assert!(matches!(refusal, Error::File(_)), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("forecasts.json"),
		"said {refusal}"
	);
}

#[rstest]
#[case::a_field_it_does_not_define(
	r#"{"unit": "thousands-of-tokens", "byEffort": {}, "status": "stub"}"#,
	"status"
)]
#[case::a_negative_count(
	r#"{"unit": "thousands-of-tokens", "byEffort": {"low": {"input": -1, "output": 0}}}"#,
	"-1"
)]
#[case::a_count_that_is_not_whole(
	r#"{"unit": "thousands-of-tokens", "byEffort": {"low": {"input": 1.5, "output": 0}}}"#,
	"1.5"
)]
fn a_forecast_nsx_cannot_take_as_written_is_a_stop(
	#[case] written: &str,
	#[case] named: &str,
) {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);

	std::fs::create_dir_all(layout.forecasts.parent().unwrap()).unwrap();
	std::fs::write(&layout.forecasts, written).unwrap();

	let refusal = Forecast::load(&root, &layout).unwrap_err();

	assert!(
		matches!(refusal, Error::Forecasts { .. }),
		"was {refusal:?}"
	);
	assert!(refusal.to_string().contains(named), "said {refusal}");
}
