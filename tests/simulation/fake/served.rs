//! A console that answers one request on this machine and keeps what it got

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

pub async fn served(
	status: &str,
	answer: &str,
) -> (String, JoinHandle<String>) {
	let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
	let origin = format!("http://{}", listener.local_addr().unwrap());
	let reply = format!(
		"HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{answer}",
		answer.len()
	);
	let received = tokio::spawn(async move {
		let (mut stream, _peer) = listener.accept().await.unwrap();
		let mut request = Vec::new();
		let mut chunk = [0_u8; 4096];

		// The request is whole once its body is as long as its header says.
		loop {
			let read = stream.read(&mut chunk).await.unwrap();

			request.extend_from_slice(&chunk[..read]);

			let text = String::from_utf8_lossy(&request).to_lowercase();
			let Some((head, body)) = text.split_once("\r\n\r\n") else {
				continue;
			};
			let length = head
				.lines()
				.find_map(|line| line.strip_prefix("content-length: "))
				.and_then(|length| length.trim().parse::<usize>().ok())
				.unwrap_or(0);

			if body.len() >= length || read == 0 {
				break;
			}
		}

		stream.write_all(reply.as_bytes()).await.unwrap();
		stream.shutdown().await.unwrap();

		String::from_utf8_lossy(&request).into_owned()
	});

	(origin, received)
}
