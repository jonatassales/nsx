//! The search ranks episodes by cosine similarity and returns the top six

use nsx::files::{layout::Layout, root::Root};
use nsx::rag::{embedder::Embedder, index::Index};
use nsx::request::ticket::Ticket;

use crate::rag::leaning::leaning;
use crate::simulation::{indexed::indexed, project::project};

#[test]
fn the_fixtures_episodes_give_six_candidates_from_0_81_to_0_42() {
	let project = project();
	let root = Root {
		path: project.path().to_path_buf(),
	};
	let id = "0031-TICKET-book-a-demo-runtime";
	let found = Ticket::resolve(&root, id).unwrap();
	let written = std::fs::read_to_string(project.path().join(&found)).unwrap();
	let ticket = Ticket::parse(id, &found, &written).unwrap();

	indexed(project.path());

	let index = Index::load(&root, &Layout::under(&root)).unwrap().unwrap();
	let query = Embedder.embed(&ticket.intent(&written));
	let ranked = index
		.search(&query)
		.into_iter()
		.map(|candidate| (candidate.id, candidate.similarity, candidate.kept))
		.collect::<Vec<_>>();

	assert_eq!(
		ranked,
		[
			("ep-0412", 0.81, true),
			("ep-0388", 0.74, true),
			("ep-0351", 0.66, true),
			("ep-0297", 0.52, false),
			("ep-0240", 0.47, false),
			("ep-0188", 0.42, false),
		]
		.map(|(id, similarity, kept)| (String::from(id), similarity, kept))
	);
}

#[test]
fn the_six_most_similar_come_back_in_descending_order() {
	let index = leaning(&[
		("ep-0001", 0.10),
		("ep-0002", 0.81),
		("ep-0003", 0.42),
		("ep-0004", 0.74),
		("ep-0005", 0.47),
		("ep-0006", 0.66),
		("ep-0007", 0.52),
		("ep-0008", 0.05),
	]);

	let candidates = index.search(&[1.0, 0.0]);
	let ranked = candidates
		.iter()
		.map(|candidate| {
			(candidate.id.as_str(), candidate.similarity, candidate.kept)
		})
		.collect::<Vec<_>>();

	assert_eq!(
		ranked,
		[
			("ep-0002", 0.81, true),
			("ep-0004", 0.74, true),
			("ep-0006", 0.66, true),
			("ep-0007", 0.52, false),
			("ep-0005", 0.47, false),
			("ep-0003", 0.42, false),
		]
	);
	assert!(
		candidates
			.iter()
			.all(|candidate| candidate.reason.is_none())
	);
}

#[test]
fn ties_keep_a_stable_order_by_ascending_id() {
	let index = leaning(&[
		("ep-0030", 0.60),
		("ep-0010", 0.60),
		("ep-0020", 0.60),
		("ep-0005", 0.90),
	]);

	let ids = index
		.search(&[1.0, 0.0])
		.into_iter()
		.map(|candidate| candidate.id)
		.collect::<Vec<_>>();

	assert_eq!(ids, ["ep-0005", "ep-0010", "ep-0020", "ep-0030"]);
}

#[test]
fn the_verdict_follows_the_similarity_as_it_is_shown() {
	let index = leaning(&[
		("ep-0001", 0.5549),
		("ep-0002", 0.5451),
		("ep-0003", 0.5449),
	]);

	let shown = index
		.search(&[1.0, 0.0])
		.into_iter()
		.map(|candidate| (candidate.similarity, candidate.kept))
		.collect::<Vec<_>>();

	assert_eq!(shown, [(0.55, true), (0.55, true), (0.54, false)]);
}

#[test]
fn a_similarity_just_below_zero_shows_no_sign() {
	let index = leaning(&[("ep-0001", -0.004)]);

	let candidates = index.search(&[1.0, 0.0]);

	assert_eq!(format!("{:.2}", candidates[0].similarity), "0.00");
	assert!(candidates[0].similarity.is_sign_positive());
}

#[test]
fn a_query_with_no_word_is_similar_to_nothing() {
	let index = leaning(&[("ep-0001", 0.90)]);

	let candidates = index.search(&[0.0, 0.0]);

	assert_eq!(candidates[0].similarity, 0.0);
	assert!(!candidates[0].kept);
}

#[test]
fn an_index_with_no_episode_gives_no_candidate() {
	assert!(leaning(&[]).search(&[1.0, 0.0]).is_empty());
}
