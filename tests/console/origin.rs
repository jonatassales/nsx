//! The console's origin: where nsx will send a secret

use nsx::console::origin::origin;
use rstest::rstest;

#[rstest]
#[case::the_published_console("https://langdrift.md", "https://langdrift.md")]
#[case::a_closing_slash("https://langdrift.md/", "https://langdrift.md")]
#[case::a_port("https://console.example:8443", "https://console.example:8443")]
#[case::this_machine_by_name("http://localhost:3001", "http://localhost:3001")]
#[case::this_machine_by_address(
	"http://127.0.0.1:3001",
	"http://127.0.0.1:3001"
)]
#[case::this_machine_by_v6_address("http://[::1]:3001", "http://[::1]:3001")]
fn an_address_nsx_will_send_a_secret_to_gives_its_origin(
	#[case] address: &str,
	#[case] expected: &str,
) {
	assert_eq!(origin(address).as_deref(), Some(expected));
}

#[rstest]
#[case::plain_http_elsewhere("http://langdrift.md")]
#[case::a_name_that_only_starts_like_this_machine(
	"http://localhost.example:3001"
)]
#[case::a_user("https://owner@langdrift.md")]
#[case::a_password("https://owner:pass@langdrift.md")]
#[case::a_path("https://langdrift.md/api/mcp")]
#[case::a_query("https://langdrift.md/?team=1")]
#[case::a_fragment("https://langdrift.md/#top")]
#[case::another_scheme("ftp://langdrift.md")]
#[case::no_address_at_all("langdrift.md")]
#[case::nothing("")]
fn any_other_address_is_refused(#[case] address: &str) {
	assert_eq!(origin(address), None);
}
