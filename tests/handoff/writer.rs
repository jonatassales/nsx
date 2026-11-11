//! `plan.json` fills section by section and is whole and valid after each

use std::path::PathBuf;

use nsx::contract::{
	analysis::Analysis, authorization::Authorization, cards::Cards,
	decode::decode, documents::Documents, plan::Plan, schema::schema,
	ticket::Ticket,
};
use nsx::files::root::Root;
use nsx::handoff::{error::Error, writer::Writer};
use serde_json::Value;

#[test]
fn after_each_section_the_file_is_valid_and_holds_every_earlier_one() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let file = project.path().join(".langdrift/runtimes/notes/plan.json");
	let mut writer = Writer {
		file: file.clone(),
		cards: Cards::default(),
	};
	let header = Ticket {
		id: String::from("notes"),
		number: None,
		slug: String::from("notes"),
		status: Some(String::from("ready-for-agent")),
		title: Some(String::from("Notes")),
		links: None,
	};
	let landed = |writer: &Writer| {
		let written = std::fs::read_to_string(&file).unwrap();
		let cards = decode::<Cards>(&written).unwrap();
		let document = serde_json::from_str::<Value>(&written).unwrap();

		assert!(jsonschema::is_valid(&schema::<Cards>(), &document));
		assert_eq!(cards, writer.cards);

		cards
	};

	writer
		.write(&root, |cards| cards.ticket = Some(header))
		.unwrap();

	let first = landed(&writer);

	writer
		.write(&root, |cards| {
			cards.analysis = Some(Analysis {
				documents: Some(Documents::default()),
				..Analysis::default()
			})
		})
		.unwrap();

	let second = landed(&writer);

	writer
		.write(&root, |cards| {
			cards.plan = Some(Plan {
				authorization: Some(Authorization::Pending),
				..Plan::default()
			})
		})
		.unwrap();

	let third = landed(&writer);

	assert!(first.ticket.is_some() && first.analysis.is_none());
	assert!(second.ticket.is_some() && second.analysis.is_some());
	assert!(second.plan.is_none());
	assert!(third.ticket.is_some() && third.analysis.is_some());
	assert!(third.plan.is_some());
}

#[test]
fn a_write_leaves_one_file_and_no_draft() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let folder = project.path().join(".langdrift/runtimes/notes");
	let mut writer = Writer {
		file: folder.join("plan.json"),
		cards: Cards::default(),
	};

	writer.write(&root, |_cards| {}).unwrap();
	writer.write(&root, |_cards| {}).unwrap();

	let names = std::fs::read_dir(&folder)
		.unwrap()
		.map(|entry| entry.unwrap().file_name())
		.collect::<Vec<_>>();

	assert_eq!(names, ["plan.json"]);
	assert_eq!(
		std::fs::read_to_string(folder.join("plan.json")).unwrap(),
		"{}\n"
	);
}

#[cfg(unix)]
#[test]
fn a_runtime_folder_that_is_a_link_is_refused() {
	let elsewhere = tempfile::tempdir().unwrap();
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let runtimes = project.path().join(".langdrift/runtimes");
	let mut writer = Writer {
		file: runtimes.join("notes/plan.json"),
		cards: Cards::default(),
	};

	std::fs::create_dir_all(&runtimes).unwrap();
	std::os::unix::fs::symlink(elsewhere.path(), runtimes.join("notes")).unwrap();

	let refusal = writer.write(&root, |_cards| {}).unwrap_err();

	assert!(matches!(refusal, Error::File(_)), "was {refusal:?}");
	assert!(
		std::fs::read_dir(elsewhere.path())
			.unwrap()
			.next()
			.is_none()
	);
}

#[test]
fn a_file_outside_the_project_is_refused() {
	let elsewhere = tempfile::tempdir().unwrap();
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let mut writer = Writer {
		file: elsewhere.path().join("plan.json"),
		cards: Cards::default(),
	};

	let refusal = writer.write(&root, |_cards| {}).unwrap_err();

	assert!(matches!(refusal, Error::File(_)), "was {refusal:?}");
	assert!(!elsewhere.path().join("plan.json").exists());
}
