//! A sink that looks at `plan.json` each time a line prints
//!
//! It keeps, for every line, the line's id and the card data on disk at that
//! moment, so a test can say which line revealed which field.

mod print;

use std::cell::RefCell;
use std::path::PathBuf;

use serde_json::Value;

pub struct Witness {
	pub file: PathBuf,
	pub seen: RefCell<Vec<(String, Value)>>,
}
