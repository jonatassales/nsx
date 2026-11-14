//! What a command printed, line by line

pub fn lines(printed: &[u8]) -> Vec<String> {
	String::from_utf8_lossy(printed)
		.lines()
		.map(String::from)
		.collect()
}
