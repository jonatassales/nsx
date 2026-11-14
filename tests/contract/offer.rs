//! The team's offer read strictly

use nsx::contract::{decode::decode, offer::Offer, provider::Provider};

use crate::contract::sample::sample;

#[test]
fn the_simulations_team_offers_three_models_at_three_efforts() {
	let offer = decode::<Offer>(&sample("offer/offer.sample.json")).unwrap();
	let [offered] = offer.providers.as_slice() else {
		panic!("the sample offers one provider");
	};

	assert_eq!(offered.provider, Provider::Anthropic);
	assert_eq!(offered.models.len(), 3);
	assert_eq!(offered.efforts, ["low", "medium", "high"]);
}

#[test]
fn a_provider_outside_the_three_or_a_field_it_does_not_define_is_refused() {
	let elsewhere =
		r#"{"providers": [{"provider": "mistral", "models": [], "efforts": []}]}"#;
	let widened = r#"{"providers": [], "budget": 100}"#;

	assert!(decode::<Offer>(elsewhere).is_err());
	assert!(decode::<Offer>(widened).is_err());
	assert!(decode::<Offer>("{}").is_err());
}
