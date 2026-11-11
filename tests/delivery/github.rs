//! git and gh, as the forge the run asks

use nsx::checks::standing::Standing;
use nsx::delivery::{error::Error, forge::Forge};
use nsx::process::output::Output;
use rstest::rstest;

use crate::delivery::{headed::headed, reached::reached};
use crate::process::{said::said, started::started};
use crate::simulation::fake::{forged::COMMIT, programs::Programs};

const BRANCH: &str = "feat/0031-book-a-demo-runtime";

const OPEN: &str = r#"{
	"number": 19,
	"title": "feat: notes",
	"state": "OPEN",
	"isDraft": true
}"#;

#[tokio::test]
async fn a_branch_the_repository_lacks_is_started() {
	let programs = Programs::default();

	programs.scripted.borrow_mut().push_back(said(1, ""));

	reached(&programs).start(BRANCH).await.unwrap();

	assert_eq!(
		started(&programs),
		[
			format!("git rev-parse --verify --quiet refs/heads/{BRANCH}"),
			format!("git checkout -b {BRANCH}"),
		]
	);
}

#[tokio::test]
async fn a_branch_an_earlier_run_left_is_checked_out() {
	let programs = Programs::default();

	reached(&programs).start(BRANCH).await.unwrap();

	assert_eq!(started(&programs)[1], format!("git checkout {BRANCH}"));
}

#[tokio::test]
async fn a_branch_that_cannot_be_checked_out_is_a_stop() {
	let programs = Programs::default();

	programs
		.scripted
		.borrow_mut()
		.extend([said(0, ""), said(1, "")]);

	let refusal = reached(&programs).start(BRANCH).await.unwrap_err();

	assert_eq!(
		refusal.to_string(),
		"git checkout ended with the exit status 1."
	);
}

#[tokio::test]
async fn the_push_sets_the_upstream_and_gives_back_the_whole_hash() {
	let programs = Programs::default();

	programs
		.scripted
		.borrow_mut()
		.extend([said(0, ""), said(0, &format!("{COMMIT}\n"))]);

	let commit = reached(&programs).push(BRANCH).await.unwrap();

	assert_eq!(commit, COMMIT);
	assert_eq!(
		started(&programs),
		[
			format!("git push --set-upstream origin {BRANCH}"),
			format!("git rev-parse --verify refs/heads/{BRANCH}"),
		]
	);
}

#[tokio::test]
async fn a_refused_push_is_a_stop_that_quotes_nothing_git_said() {
	let programs = Programs::default();

	programs.scripted.borrow_mut().push_back(Output {
		complained: b"fatal: https://token@host/repository denied".to_vec(),
		..said(128, "")
	});

	let refusal = reached(&programs).push(BRANCH).await.unwrap_err();

	assert_eq!(
		refusal.to_string(),
		"git push ended with the exit status 128."
	);
	assert_eq!(started(&programs).len(), 1);
}

#[tokio::test]
async fn a_branch_with_no_pull_request_gets_a_draft_one() {
	let programs = Programs::default();

	programs.scripted.borrow_mut().extend([
		said(1, ""),
		said(0, "https://host/pull/19\n"),
		said(0, OPEN),
	]);

	let opened = reached(&programs).open(BRANCH).await.unwrap();

	assert_eq!(opened, (19, String::from("feat: notes")));
	assert_eq!(
		started(&programs),
		[
			format!("gh pr view {BRANCH} --json number,title,state,isDraft"),
			format!("gh pr create --draft --fill-first --head {BRANCH}"),
			format!("gh pr view {BRANCH} --json number,title,state,isDraft"),
		]
	);
}

#[tokio::test]
async fn the_open_pull_request_an_earlier_run_left_is_used() {
	let programs = Programs::default();

	programs.scripted.borrow_mut().push_back(said(0, OPEN));

	let opened = reached(&programs).open(BRANCH).await.unwrap();

	assert_eq!(opened.0, 19);
	assert_eq!(started(&programs).len(), 1);
}

#[tokio::test]
async fn a_merged_pull_request_is_not_used_again() {
	let programs = Programs::default();

	programs.scripted.borrow_mut().extend([
		said(
			0,
			r#"{"number": 12, "title": "old", "state": "MERGED", "isDraft": false}"#,
		),
		said(0, ""),
		said(
			0,
			r#"{"number": 20, "title": "new", "state": "OPEN", "isDraft": true}"#,
		),
	]);

	let opened = reached(&programs).open(BRANCH).await.unwrap();

	assert_eq!(opened.0, 20);
	assert_eq!(started(&programs).len(), 3);
}

#[tokio::test]
async fn an_open_pull_request_someone_marked_ready_goes_back_to_a_draft() {
	let programs = Programs::default();

	programs.scripted.borrow_mut().push_back(said(
		0,
		r#"{"number": 19, "title": "notes", "state": "OPEN", "isDraft": false}"#,
	));

	let opened = reached(&programs).open(BRANCH).await.unwrap();

	assert_eq!(opened.0, 19);
	assert_eq!(
		started(&programs),
		[
			format!("gh pr view {BRANCH} --json number,title,state,isDraft"),
			format!("gh pr ready {BRANCH} --undo"),
		]
	);
}

#[rstest]
#[case::not_json("the pull request", false)]
#[case::no_number(
	r#"{"title": "notes", "state": "OPEN", "isDraft": true}"#,
	true
)]
#[case::no_title(r#"{"number": 19, "state": "OPEN", "isDraft": true}"#, true)]
#[case::no_state(r#"{"number": 19, "title": "notes", "isDraft": true}"#, true)]
#[case::no_draft(r#"{"number": 19, "title": "notes", "state": "OPEN"}"#, true)]
#[tokio::test]
async fn what_gh_says_of_a_pull_request_is_read_strictly(
	#[case] viewed: &str,
	#[case] lacking: bool,
) {
	let programs = Programs::default();

	programs.scripted.borrow_mut().extend([
		said(1, ""),
		said(0, ""),
		said(0, viewed),
	]);

	let refusal = reached(&programs).open(BRANCH).await.unwrap_err();

	assert_eq!(
		matches!(refusal, Error::Lacking { .. }),
		lacking,
		"was {refusal:?}"
	);
	assert_eq!(
		matches!(refusal, Error::Answer { .. }),
		!lacking,
		"was {refusal:?}"
	);
}

#[tokio::test]
async fn each_check_stands_where_its_bucket_says() {
	let programs = Programs::default();

	// gh ends with 8 while a check is pending: the list is still the answer.
	programs.scripted.borrow_mut().extend([
		headed(COMMIT),
		said(
			8,
			r#"[
			{"name": "build", "bucket": "pass"},
			{"name": "check", "bucket": "pending"},
			{"name": "test", "bucket": "fail"},
			{"name": "lint", "bucket": "cancel"},
			{"name": "docs", "bucket": "skipping"}
		]"#,
		),
	]);

	let listed = reached(&programs).checks(19, COMMIT).await.unwrap();
	let checks = listed.unwrap();
	let standings = checks
		.iter()
		.map(|check| (check.name.as_str(), check.standing))
		.collect::<Vec<_>>();

	assert_eq!(
		standings,
		[
			("build", Standing::Passed),
			("check", Standing::Running),
			("test", Standing::Failed),
			("lint", Standing::Failed),
			("docs", Standing::Failed),
		]
	);
	assert_eq!(
		started(&programs),
		[
			"gh pr view 19 --json headRefOid",
			"gh pr checks 19 --json name,bucket",
		]
	);
}

#[tokio::test]
async fn a_pull_request_with_no_check_yet_lists_none() {
	let programs = Programs::default();

	programs.scripted.borrow_mut().extend([
		headed(COMMIT),
		Output {
			complained: b"no checks reported on the 'feat/notes' branch".to_vec(),
			..said(1, "")
		},
	]);

	let checks = reached(&programs).checks(19, COMMIT).await.unwrap();

	assert_eq!(checks, Some(Vec::new()));
}

#[tokio::test]
async fn gh_failing_with_nothing_to_read_is_a_stop_and_not_an_empty_list() {
	let programs = Programs::default();

	programs.scripted.borrow_mut().extend([
		headed(COMMIT),
		Output {
			complained: b"HTTP 401: Bad credentials".to_vec(),
			..said(1, "")
		},
	]);

	let refusal = reached(&programs).checks(19, COMMIT).await.unwrap_err();

	assert_eq!(refusal.to_string(), "gh pr ended with the exit status 1.");
}

#[rstest]
#[case::a_word_nsx_does_not_know(r#"[{"name": "build", "bucket": "queued"}]"#)]
#[case::no_bucket(r#"[{"name": "build"}]"#)]
#[case::no_name(r#"[{"bucket": "pass"}]"#)]
#[tokio::test]
async fn a_check_nsx_cannot_place_is_a_stop(#[case] listed: &str) {
	let programs = Programs::default();

	programs
		.scripted
		.borrow_mut()
		.extend([headed(COMMIT), said(0, listed)]);

	let refusal = reached(&programs).checks(19, COMMIT).await.unwrap_err();

	assert!(matches!(refusal, Error::Lacking { .. }), "was {refusal:?}");
}

#[tokio::test]
async fn marking_ready_asks_gh_for_that_pull_request_and_never_merges() {
	let programs = Programs::default();
	let github = reached(&programs);
	let forge: &dyn Forge = &github;

	forge.ready(19).await.unwrap();

	assert_eq!(started(&programs), ["gh pr ready 19"]);
	assert!(!started(&programs).iter().any(|tool| tool.contains("merge")));
}

#[tokio::test]
async fn a_pull_request_at_another_commit_has_no_checks_of_the_one_pushed() {
	let programs = Programs::default();

	programs.scripted.borrow_mut().extend([
		headed("0000000000000000000000000000000000000000"),
		said(0, r#"[{"name": "build", "bucket": "pass"}]"#),
	]);

	let checks = reached(&programs).checks(19, COMMIT).await.unwrap();

	assert_eq!(checks, None);
	assert_eq!(started(&programs), ["gh pr view 19 --json headRefOid"]);
}

#[tokio::test]
async fn a_pull_request_whose_head_gh_does_not_say_is_a_stop() {
	let programs = Programs::default();

	programs.scripted.borrow_mut().push_back(said(0, "{}"));

	let refusal = reached(&programs).checks(19, COMMIT).await.unwrap_err();

	assert!(matches!(refusal, Error::Lacking { .. }), "was {refusal:?}");
	assert_eq!(started(&programs).len(), 1);
}
