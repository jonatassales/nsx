//! Asking the console's judge
//!
//! One function: it takes a question, posts it to the console's judge and
//! gives back the answer's text.
//!
//! ## Reads and writes
//!
//! - posts to `<origin>/api/judge`, with the secret as the bearer: the
//!   question's kind, the model, nsx's instructions and facts, the most
//!   tokens the answer may take and the version of nsx's words
//!
//! ## Rules
//!
//! - The kind is the question's name: `criteria`, `episodes` or
//!   `suggestion`.
//! - An answer the console's judge declined, a status that is not 200 or a
//!   body without the answer's text stops the plan with one sentence that
//!   carries the console's reason and never the secret.
//! - What the answer says is checked by whoever asked.

use std::future::Future;
use std::pin::Pin;

use serde_json::{Value, json};

use crate::judgment::{
	client::{ANSWER_TOKENS, Client, MODEL, PATIENCE, WORDS},
	error::Error,
	judge::Judge,
	question::Question,
};

impl Judge for Client {
	fn ask<'call>(
		&'call self,
		question: &'call Question<'call>,
	) -> Pin<Box<dyn Future<Output = Result<String, Error>> + 'call>> {
		Box::pin(async move {
			let unavailable = |cause: String| Error::Unavailable {
				cause,
			};
			let failed =
				|cause: reqwest::Error| unavailable(cause.without_url().to_string());
			let (system, prompt) = question.prompt();
			let asked = json!({
				"kind": question.name(),
				"model": MODEL,
				"system": system,
				"prompt": prompt,
				"maxTokens": ANSWER_TOKENS,
				"promptVersion": WORDS,
			});

			let answered = self
				.http
				.post(format!("{}/api/judge", self.connection.origin))
				.timeout(PATIENCE)
				.bearer_auth(&self.connection.secret)
				.json(&asked)
				.send()
				.await
				.map_err(failed)?;

			let status = answered.status();

			let body = answered.json::<Value>().await.map_err(failed)?;

			let stated = |field: &str| body.get(field).and_then(Value::as_str);
			let declined = body.get("refused").and_then(Value::as_bool) == Some(true);
			let reason = stated("reason").or(stated("code"));

			if !status.is_success() || declined {
				let said = reason.map(|reason| format!(" {reason}"));

				return Err(unavailable(format!(
					"the console answered {status}{}",
					said.unwrap_or_default()
				)));
			}

			stated("text").map(String::from).ok_or_else(|| {
				unavailable(String::from("the console's answer carries no text"))
			})
		})
	}
}
