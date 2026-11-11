//! The dev ticket as the run hands it to the person

use nsx::seat::part::Part;

pub fn waiting() -> Part {
	Part {
		state: String::from("active"),
		closed_by: None,
		paid_calls: None,
	}
}
