//! A program that ended with a status and printed nothing

use nsx::process::output::Output;

pub fn ended(status: i32) -> Output {
	Output {
		status: Some(status),
		printed: Vec::new(),
		complained: Vec::new(),
	}
}
