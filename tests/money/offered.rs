//! An offer from Anthropic alone: the given models at the given efforts

use nsx::contract::{offer::Offer, offered::Offered, provider::Provider};

pub fn offered(models: &[&str], efforts: &[&str]) -> Offer {
	let owned =
		|words: &[&str]| words.iter().map(|word| String::from(*word)).collect();

	Offer {
		providers: vec![Offered {
			provider: Provider::Anthropic,
			models: owned(models),
			efforts: owned(efforts),
		}],
	}
}
