//! No control character reaches the terminal

use assert_cmd::Command;
use nsx::session::{
	line::Line, sink::Sink, terminal::Terminal, title::Title, window::Window,
};
use rstest::rstest;

#[rstest]
#[case::a_line_break("use case: a demo\nconsole › forged")]
#[case::an_escape("use case: a demo\u{1b}[2K")]
#[case::a_carriage_return("use case: a demo\rconsole › forged")]
#[case::a_bell("use case: a demo\u{7}")]
fn a_line_that_carries_a_control_character_is_not_printed(#[case] text: &str) {
	let terminal = Terminal {
		toned: false,
	};

	let stop = terminal.print(&Line::comment("readPrd", text)).unwrap_err();

	assert_eq!(
		stop.sentence,
		"nsx will not print the line readPrd: it carries a control character."
	);
}

#[test]
fn a_folder_name_with_a_control_character_never_titles_the_window() {
	let terminal = Terminal {
		toned: true,
	};
	let title = Title {
		project: String::from("demo\u{7}\u{1b}]0;forged"),
	};

	let stop = terminal.title(&title).unwrap_err();

	assert!(
		stop.sentence.contains("will not title the window"),
		"said {stop:?}"
	);
}

#[test]
fn a_stop_shows_a_control_character_as_its_escape() {
	let project = tempfile::tempdir().unwrap();

	std::fs::create_dir(project.path().join(".langdrift")).unwrap();

	let output = Command::cargo_bin("nsx")
		.unwrap()
		.args(["agent", "plan", "0031\u{1b}[2K-TICKET-forged"])
		.current_dir(project.path())
		.output()
		.unwrap();
	let said = String::from_utf8(output.stderr).unwrap();

	assert_eq!(output.status.code(), Some(1));
	assert!(said.contains("\\u{1b}[2K"), "said {said:?}");
	assert!(!said.contains('\u{1b}'), "said {said:?}");
	assert!(
		said.ends_with(".\n") && said.lines().count() == 1,
		"said {said:?}"
	);
}
