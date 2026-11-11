//! The one answer, whatever the component

use nsx::components::{
	component::Component, error::Error, image::Image, resolver::Resolver,
};

use crate::simulation::fake::answering::Answering;

impl Resolver for Answering {
	fn resolve(
		&self,
		_runtime: &str,
		_revision: i64,
		_component: &Component,
	) -> Result<Image, Error> {
		Ok(Image(String::from(self.answer)))
	}
}
