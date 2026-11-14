//! The console and the secret an end-to-end run needs, from the environment

const NEEDS: &str = "The end-to-end run needs MCP_CONSOLE_URL and LANGDRIFT_SECRET: no console is a default.";

pub fn connection() -> (String, String) {
	let named =
		|name: &str| std::env::var(name).unwrap_or_else(|_| panic!("{NEEDS}"));

	(named("MCP_CONSOLE_URL"), named("LANGDRIFT_SECRET"))
}
