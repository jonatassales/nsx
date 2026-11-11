//! The plan's ticket card, with the number a test gives it

use nsx::contract::ticket::Ticket;

pub fn ticketed(number: Option<&str>) -> Ticket {
	Ticket {
		id: String::from("0031-TICKET-book-a-demo-runtime"),
		number: number.map(String::from),
		slug: String::from("book-a-demo-runtime"),
		status: None,
		title: None,
		links: None,
	}
}
