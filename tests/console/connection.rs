//! The connection: recorded in a private file Git does not track, read back

use std::path::PathBuf;
use std::process::Command;

use nsx::console::{connection::Connection, error::Error};
use nsx::files::{layout::Layout, root::Root};
use nsx::process::local::Local;
use rstest::rstest;

const SECRET: &str =
	"dft_0000000000000000000000000000000000000000000000000000000000000000";

#[tokio::test]
async fn the_connection_is_recorded_privately_and_reads_back() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let offered = Connection {
		origin: String::from("http://localhost:3001/"),
		secret: String::from(SECRET),
	};

	offered.record(&root, &layout, &Local).await.unwrap();

	let read = Connection::load(&root, &layout).unwrap().unwrap();

	assert_eq!(read.origin, "http://localhost:3001");
	assert_eq!(read.secret, SECRET);

	#[cfg(unix)]
	{
		use std::os::unix::fs::PermissionsExt;

		let mode = |path: &std::path::Path| {
			std::fs::metadata(path).unwrap().permissions().mode() & 0o777
		};

		assert_eq!(mode(&layout.credentials), 0o600);
		assert_eq!(mode(layout.credentials.parent().unwrap()), 0o700);
	}
}

#[test]
fn a_project_that_recorded_no_secret_has_no_connection() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let read = Connection::load(&root, &Layout::under(&root)).unwrap();

	assert!(read.is_none());
}

#[tokio::test]
async fn a_credentials_file_git_tracks_is_refused_and_left_as_it_was() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let git = |arguments: &[&str]| {
		let ended = Command::new("git")
			.args(arguments)
			.current_dir(project.path())
			.output()
			.unwrap();

		assert!(ended.status.success(), "git {arguments:?}");
	};
	let offered = Connection {
		origin: String::from("https://langdrift.md"),
		secret: String::from(SECRET),
	};

	std::fs::create_dir_all(layout.credentials.parent().unwrap()).unwrap();
	std::fs::write(&layout.credentials, "{}").unwrap();
	git(&["init", "--quiet"]);
	git(&["add", "--force", "--", ".langdrift"]);

	let refusal = offered.record(&root, &layout, &Local).await.unwrap_err();

	assert!(matches!(refusal, Error::Tracked { .. }), "was {refusal:?}");
	assert!(!refusal.to_string().contains(SECRET));
	assert_eq!(std::fs::read_to_string(&layout.credentials).unwrap(), "{}");
}

#[cfg(unix)]
#[tokio::test]
async fn a_credentials_file_others_can_read_is_refused() {
	use std::fs::Permissions;
	use std::os::unix::fs::PermissionsExt;

	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let offered = Connection {
		origin: String::from("https://langdrift.md"),
		secret: String::from(SECRET),
	};

	offered.record(&root, &layout, &Local).await.unwrap();
	std::fs::set_permissions(&layout.credentials, Permissions::from_mode(0o644))
		.unwrap();

	let refusal = Connection::load(&root, &layout).err().unwrap();

	assert!(matches!(refusal, Error::Exposed { .. }), "was {refusal:?}");
	assert!(!refusal.to_string().contains(SECRET));
}

#[tokio::test]
async fn an_address_or_a_secret_nsx_will_not_use_is_refused_unrecorded() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let elsewhere = Connection {
		origin: String::from("http://langdrift.md"),
		secret: String::from(SECRET),
	};
	let spaced = Connection {
		origin: String::from("https://langdrift.md"),
		secret: String::from("dft_00 11"),
	};

	let address = elsewhere.record(&root, &layout, &Local).await.unwrap_err();
	let secret = spaced.record(&root, &layout, &Local).await.unwrap_err();

	assert!(matches!(address, Error::Origin), "was {address:?}");
	assert!(matches!(secret, Error::Secret), "was {secret:?}");
	assert!(!secret.to_string().contains("dft_00"));
	assert!(!layout.credentials.exists());
}

#[rstest]
#[case::the_secret_alone(r#""sk-live-PASTED""#, "sk-live-PASTED")]
#[case::a_secret_that_is_a_number(
	r#"{"origin": "https://langdrift.md", "secret": 4815162342}"#,
	"4815162342"
)]
#[case::an_address_with_a_password(
	r#"{"origin": "https://owner:hunter2@langdrift.md", "secret": "dft_00"}"#,
	"hunter2"
)]
fn a_credentials_file_that_does_not_read_is_refused_without_quoting_it(
	#[case] written: &str,
	#[case] private: &str,
) {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);

	nsx::files::sealed::sealed(&layout.credentials, written.as_bytes()).unwrap();

	let refusal = Connection::load(&root, &layout).err().unwrap();

	assert!(!refusal.to_string().contains(private), "said {refusal}");
}

#[tokio::test]
async fn the_private_state_is_ignored_by_git_once_a_connection_is_recorded() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let offered = Connection {
		origin: String::from("https://langdrift.md"),
		secret: String::from(SECRET),
	};
	let git = |arguments: &[&str]| {
		Command::new("git")
			.args(arguments)
			.current_dir(project.path())
			.output()
			.unwrap()
	};

	git(&["init", "--quiet"]);
	offered.record(&root, &layout, &Local).await.unwrap();

	let status = git(&["status", "--porcelain", "--untracked-files=all"]);
	let listed = String::from_utf8_lossy(&status.stdout);

	assert!(status.status.success());
	assert!(!listed.contains("console.json"), "listed {listed}");
}
