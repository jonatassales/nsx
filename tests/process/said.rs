//! What a tool printed, ending with the status a test gives it

use nsx::process::output::Output;

pub fn said(status: i32, printed: &str) -> Output {
	Output {
		status: Some(status),
		printed: printed.as_bytes().to_vec(),
		complained: Vec::new(),
	}
}
