//! The embedder gives a text the same unit vector every time

use nsx::rag::embedder::{DIMENSIONS, Embedder, ID};
use rstest::rstest;

#[test]
fn a_text_embeds_into_one_unit_vector_of_the_embedders_dimensions() {
	let vector = Embedder.embed("Run the book-a-demo request as a microruntime");
	let length = vector
		.iter()
		.map(|cell| f64::from(*cell).powi(2))
		.sum::<f64>()
		.sqrt();

	assert_eq!(ID, "local-hashed-tfidf-1");
	assert_eq!(vector.len(), DIMENSIONS);
	assert!((length - 1.0).abs() < 1e-6, "length {length}");
	assert_eq!(
		vector,
		Embedder.embed("Run the book-a-demo request as a microruntime")
	);
}

#[rstest]
#[case::case_does_not_count("Sign-Up API", "sign-up api")]
#[case::punctuation_parts_words("sign-up, api!", "sign-up api")]
#[case::a_trailing_dash_is_dropped("sign-up- api", "sign-up api")]
#[case::two_joiners_part_words("go..mod", "go mod")]
#[case::a_dot_keeps_a_name_whole("read go.mod now", "read  go.mod   now")]
fn texts_with_the_same_words_embed_alike(
	#[case] left: &str,
	#[case] right: &str,
) {
	assert_eq!(Embedder.embed(left), Embedder.embed(right));
}

#[rstest]
#[case::a_dash_joins_one_word("sign-up", "sign up")]
#[case::order_counts("api sign-up", "sign-up api")]
fn texts_with_other_words_embed_apart(#[case] left: &str, #[case] right: &str) {
	assert_ne!(Embedder.embed(left), Embedder.embed(right));
}

#[test]
fn a_text_with_no_word_embeds_into_the_zero_vector() {
	let vector = Embedder.embed(" — … ");

	assert!(vector.iter().all(|cell| *cell == 0.0));
}

#[test]
fn one_word_lands_whole_in_one_dimension() {
	let vector = Embedder.embed("microruntime");
	let filled = vector
		.iter()
		.filter(|cell| **cell != 0.0)
		.collect::<Vec<_>>();

	assert_eq!(filled.len(), 1);
	assert_eq!(filled[0].abs(), 1.0);
}
