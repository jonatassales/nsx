//! The plan act: components and sizing
//!
//! The third act of the plan: the components the spec names are resolved one
//! by one, then sized inside the envelope.
//!
//! One function: it takes what the plan read before its first line, the spec
//! with the component paths it names, and the card data's writer, and plays
//! the act, each step calling the folder that owns the line. It gives back
//! the resolved components, their sizing and the revision the draft takes.
//!
//! The steps, in the simulation's order:
//!
//! - here: the `planning` comment
//! - components: take the paths the spec names (`resolve`)
//! - components: inspect each one in the spec's order, an inspect line and a
//!   row each (`inspectRust` and `rust`, `inspectNode` and `node`,
//!   `inspectNats` and `nats`, `inspectRedis` and `redis`, `inspectNext`
//!   and `next`)
//! - runtime: take the draft's revision, which each image carries
//! - components: resolve each component's image, then cross-check the ports,
//!   the images and the paths (`crossCheck`)
//! - runtime: sum the shares against the bound (`size`), then state the
//!   limits (`limits`)
//! - money: state that the runtime has no price source (`price`)
//!
//! ## Simulation
//!
//! - `planning`: `# plan · resolving the components, then sizing them inside the envelope`
//!
//! Spec 0077.

use crate::command::{
	brief::Brief, desk::Desk, plan::Plan, resolved::Resolved, session::Session,
};
use crate::components::{
	collisions::Collisions, component::Component, image::Image,
	inventory::Inventory,
};
use crate::contract::status::Status;
use crate::harness::envelope::Envelope;
use crate::request::spec::Spec;
use crate::runtime::{limits::Limits, revision::Revision, sizing::Sizing};
use crate::session::{line::Line, stop::Stop};

impl Plan {
	pub async fn resolve(
		&self,
		session: &mut Session<'_>,
		brief: &Brief,
		spec: &Spec,
		desk: &mut Desk,
	) -> Result<Resolved, Stop> {
		let root = &brief.root;
		let runtime = &brief.ticket.slug;
		let draft = brief.layout.runtimes.join(runtime).join("runtime.md");
		let revision = Revision::next(root, &draft)?;
		let bound = Envelope::of(&brief.design);
		let mut inventory = Inventory::default();

		desk.report.status = Status::Plan;

		session.sink.print(&Line::comment(
			"planning",
			"plan · resolving the components, then sizing them inside the envelope",
		))?;

		session.sink.print(&Inventory::line(&spec.components))?;
		desk
			.record(root, session.console, |cards| {
				Inventory::card(&spec.components, cards)
			})
			.await?;

		for named in &spec.components {
			let component = Component::inspect(root, named)?;

			session.sink.print(&component.line())?;

			session.sink.print(&component.row())?;

			inventory.components.push(component.clone());

			let so_far = Sizing::sum(&inventory.components, &bound);

			desk
				.record(root, session.console, |cards| {
					component.card(cards);
					so_far.proposed(cards);
				})
				.await?;
		}

		for component in &mut inventory.components {
			let image =
				Image::resolve(session.images, runtime, revision.0, component)?;

			component.image = Some(image);
		}

		let collisions = Collisions::check(&inventory.components)?;

		session.sink.print(&collisions.line())?;
		desk
			.record(root, session.console, |cards| {
				collisions.card(&inventory.components, cards);
			})
			.await?;

		let sizing = Sizing::sum(&inventory.components, &bound);

		sizing.fit()?;

		session.sink.print(&sizing.line())?;
		desk
			.record(root, session.console, |cards| sizing.card(cards))
			.await?;

		session.sink.print(&Limits.line())?;
		desk
			.record(root, session.console, |cards| Limits.card(cards))
			.await?;

		session.sink.print(&brief.prices.line())?;
		desk
			.record(root, session.console, |cards| brief.prices.card(cards))
			.await?;

		Ok(Resolved {
			inventory,
			sizing,
			revision,
		})
	}
}
