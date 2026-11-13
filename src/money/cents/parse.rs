//! Reading a dollar amount
//!
//! One function: it takes an amount as a team writes it, such as `$25.00`,
//! and gives back the cents, or nothing when the text is not such an amount.
//!
//! ## Rules
//!
//! - The amount opens with `$`. It has no cents or exactly two digits of
//!   them, as in `$25` and `$25.00`.
//! - The dollars may be grouped in threes with commas, as in `$1,250.00`, or
//!   not grouped at all. Any other comma is refused: `$25,00` is not $2,500.

use crate::money::cents::Cents;

const GROUP: usize = 3;

impl Cents {
	pub fn parse(written: &str) -> Option<Self> {
		let digits = |text: &str| {
			!text.is_empty() && text.chars().all(|digit| digit.is_ascii_digit())
		};

		let amount = written.strip_prefix('$')?;
		let (dollars, fraction) = amount.split_once('.').unwrap_or((amount, "00"));
		let groups = dollars.split(',').collect::<Vec<_>>();
		let (first, rest) = groups.split_first()?;
		let leading = digits(first) && (rest.is_empty() || first.len() <= GROUP);
		let thousands = rest
			.iter()
			.all(|group| group.len() == GROUP && digits(group));
		let cents = fraction.len() == 2 && digits(fraction);
		let well_formed = leading && thousands && cents;

		if !well_formed {
			return None;
		}

		let whole = groups.concat().parse::<i64>().ok()?;
		let part = fraction.parse::<i64>().ok()?;

		Some(Self(whole.checked_mul(100)?.checked_add(part)?))
	}
}
