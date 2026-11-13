//! Keeping the title
//!
//! One function: it takes the window's title and keeps it in place of the
//! one it held. It never stops.

use nsx::session::{stop::Stop, title::Title, window::Window};

use crate::simulation::fake::recorder::Recorder;

impl Window for Recorder {
	fn title(&self, title: &Title) -> Result<(), Stop> {
		self.title.replace(Some(title.clone()));

		Ok(())
	}
}
