//! A component's image: asked of the resolver, and checked before it is kept

use nsx::components::{derived::Derived, error::Error, image::Image};
use rstest::rstest;

use crate::components::resolved::resolved;
use crate::simulation::fake::answering::Answering;

#[rstest]
#[case::a_folder_under_services(
	"services/signup-api",
	1,
	"book-a-demo-runtime/signup-api:r1"
)]
#[case::a_later_revision("platform/nats", 7, "book-a-demo-runtime/nats:r7")]
#[case::a_path_with_a_closing_slash(
	"apps/signup/",
	2,
	"book-a-demo-runtime/signup:r2"
)]
#[case::a_folder_at_the_root("api", 1, "book-a-demo-runtime/api:r1")]
fn an_image_is_derived_from_the_runtime_the_folder_and_the_revision(
	#[case] path: &str,
	#[case] revision: i64,
	#[case] reference: &str,
) {
	let component = resolved(path, 8080, None);

	let image =
		Image::resolve(&Derived, "book-a-demo-runtime", revision, &component)
			.unwrap();

	assert_eq!(image.0, reference);
}

#[rstest]
#[case::an_empty_answer("")]
#[case::an_answer_with_a_space("demo/api r1")]
#[case::an_answer_with_an_escape("demo/api\u{1b}[2J")]
#[case::an_answer_over_two_lines("demo/api:r1\nimage: other")]
fn an_answer_that_is_not_a_reference_is_refused(#[case] answer: &'static str) {
	let images = Answering {
		answer,
	};
	let component = resolved("services/signup-api", 8080, None);

	let refusal = Image::resolve(&images, "notes", 1, &component).unwrap_err();

	assert!(matches!(refusal, Error::Image { .. }), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("services/signup-api"),
		"said {refusal}"
	);
	assert!(!refusal.to_string().contains('\u{1b}'), "said {refusal:?}");
}
