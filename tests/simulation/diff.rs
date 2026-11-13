//! What differs between a golden and what nsx printed
//!
//! Every disagreement is one message naming its line, in the golden's order:
//! a line that differs, a missing or an extra line, a kind that differs, and
//! a pending entry that excuses nothing.
//!
//! - A golden line pairs with the next printed line of its id, so a missing
//!   or an extra line never shifts the lines after it.
//! - A pending id excuses the text of the golden line it names, that line's
//!   absence, and one extra line printed with that id. It excuses nothing
//!   else, so it never hides another line's regression.
//! - A pending id never excuses a kind.
//! - A pending line that already matches, or a pending id that names no
//!   line, is a disagreement too: the list only shrinks.

use std::collections::BTreeSet;

use crate::simulation::frozen::Frozen;

pub fn diff(
	golden: &[Frozen],
	printed: &[Frozen],
	pending: &[&str],
) -> Vec<String> {
	let listed = pending.iter().copied().collect::<BTreeSet<_>>();
	let mut differences = Vec::new();
	let mut excused = BTreeSet::new();
	let mut extras = BTreeSet::new();

	let mut cursor = 0;
	let pairs = golden
		.iter()
		.map(|want| {
			let offset = printed[cursor..]
				.iter()
				.position(|line| line.id == want.id)?;
			let index = cursor + offset;

			cursor = index + 1;

			Some(index)
		})
		.collect::<Vec<_>>();

	for (position, (want, pair)) in golden.iter().zip(&pairs).enumerate() {
		let number = position + 1;
		let id = want.id.as_str();
		let is_listed = listed.contains(id);
		let got = pair.map(|index| &printed[index]);
		let differs = got.is_none_or(|got| got.text != want.text);

		if differs && is_listed {
			excused.insert(id);
		}

		let Some(got) = got else {
			if !is_listed {
				differences.push(format!(
					"line {number}, {id}, is missing: want {} {:?}",
					want.kind, want.text
				));
			}

			continue;
		};

		if differs && !is_listed {
			differences.push(format!(
				"line {number}, {id}, differs: printed {} {:?}, want {} {:?}",
				got.kind, got.text, want.kind, want.text
			));

			continue;
		}

		if got.kind != want.kind {
			differences.push(format!(
				"line {number}, {id}, prints as {}: want {} {:?}",
				got.kind, want.kind, want.text
			));
		}
	}

	let paired = pairs.iter().flatten().collect::<BTreeSet<_>>();

	for (index, extra) in printed.iter().enumerate() {
		let id = extra.id.as_str();

		if paired.contains(&index) {
			continue;
		}

		let first_of_a_listed_id = listed.contains(id) && extras.insert(id);

		if first_of_a_listed_id {
			continue;
		}

		differences.push(format!(
			"printed line {}, {id}, is extra: {} {:?}",
			index + 1,
			extra.kind,
			extra.text
		));
	}

	for (position, want) in golden.iter().enumerate() {
		let id = want.id.as_str();

		if !listed.contains(id) {
			continue;
		}

		if !excused.contains(id) {
			differences.push(format!(
				"line {}, {id}, matches but is pending: drop it from the pending list",
				position + 1
			));
		}
	}

	let named = golden
		.iter()
		.map(|line| line.id.as_str())
		.collect::<BTreeSet<_>>();

	for id in &listed {
		let excuses = named.contains(id) || extras.contains(id);

		if !excuses {
			differences.push(format!(
				"pending {id} names no line that differs: drop it from the pending list"
			));
		}
	}

	differences
}
