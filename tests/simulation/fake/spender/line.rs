//! One answer, and one more paid call in the usage record

use nsx::evidence::usage::Usage;
use nsx::flow::reader::Reader;

use crate::simulation::fake::spender::Spender;

impl Reader for Spender {
	fn line(&self) -> std::io::Result<Option<String>> {
		if let Ok(written) = std::fs::read_to_string(&self.file) {
			let mut kept: Vec<Usage> = serde_json::from_str(&written).unwrap();
			let again = kept[0].calls[0].clone();

			kept[0].calls.push(again);

			std::fs::write(&self.file, serde_json::to_vec(&kept).unwrap()).unwrap();
		}

		Ok(Some(String::from("yes")))
	}
}
