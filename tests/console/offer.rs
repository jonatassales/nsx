//! The team's offer a connected plan reads before its first line

use std::path::PathBuf;

use nsx::command::{init::Init, plan::Plan};
use nsx::contract::{offer::Offer, offered::Offered, provider::Provider};
use rstest::rstest;

use crate::money::offered::offered;
use crate::simulation::{
	fake::{
		console::Console, judge::judge, recorder::Recorder, session::session,
	},
	indexed::indexed,
	project::project,
};

const SONNET: &str = "claude-sonnet-5-5";

const HAIKU: &str = "claude-haiku-5-5";

#[tokio::test]
async fn an_offer_narrows_the_workflow_cards_spend_rows() {
	let project = project();
	let recorder = Recorder::default();
	let console = Console {
		offer: Some(offered(&[SONNET, HAIKU], &["low", "medium"])),
		..Console::default()
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let judge = judge();

	Init.execute(&mut session).await.unwrap();
	indexed(project.path());

	session.judge = &judge;
	session.console = &console;

	plan.execute(&mut session).await.unwrap();

	let reports = console.reports.borrow();
	let workflow = reports[0]
		.plan
		.as_ref()
		.and_then(|cards| cards.workflow.as_ref())
		.unwrap();
	let rows = workflow
		.spend
		.iter()
		.map(|pair| (pair.model.as_str(), pair.effort.as_str()))
		.collect::<Vec<_>>();

	assert_eq!(
		rows,
		[
			(SONNET, "low"),
			(SONNET, "medium"),
			(HAIKU, "low"),
			(HAIKU, "medium"),
		]
	);
}

#[rstest]
#[case::no_provider(
	Some(Offer {
		providers: Vec::new(),
	}),
	false
)]
#[case::no_model_nsx_prices(
	Some(Offer {
		providers: vec![Offered {
			provider: Provider::Gemini,
			models: vec![String::from("gemini-3-pro")],
			efforts: vec![String::from("medium")],
		}],
	}),
	false
)]
#[case::no_effort_nsx_knows(Some(offered(&[SONNET], &["max"])), false)]
#[case::an_offer_that_cannot_be_read(None, true)]
#[tokio::test]
async fn an_offer_that_leaves_no_candidate_stops_the_plan_before_its_first_line(
	#[case] offer: Option<Offer>,
	#[case] offer_down: bool,
) {
	let project = project();
	let recorder = Recorder::default();
	let console = Console {
		offer,
		offer_down,
		..Console::default()
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let judge = judge();

	Init.execute(&mut session).await.unwrap();
	indexed(project.path());

	let printed = recorder.lines.borrow().len();

	session.judge = &judge;
	session.console = &console;

	let stop = plan.execute(&mut session).await.unwrap_err();

	assert!(
		stop.sentence.contains("Settings → AI Models"),
		"said {}",
		stop.sentence
	);
	assert_eq!(recorder.lines.borrow().len(), printed);
	assert!(console.reports.borrow().is_empty());
}

#[tokio::test]
async fn a_default_choice_the_team_does_not_offer_stops_the_plan() {
	let project = project();
	let recorder = Recorder::default();
	let console = Console {
		offer: Some(offered(&[HAIKU], &["low", "medium"])),
		..Console::default()
	};
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let plan = Plan {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
	};
	let judge = judge();

	Init.execute(&mut session).await.unwrap();
	indexed(project.path());

	session.judge = &judge;
	session.console = &console;

	let stop = plan.execute(&mut session).await.unwrap_err();

	assert!(
		stop.sentence.ends_with(
			"its default model claude-sonnet-5-5 at the effort medium is not offered on the console's Settings → AI Models."
		),
		"said {}",
		stop.sentence
	);
	assert!(console.reports.borrow().is_empty());
}
