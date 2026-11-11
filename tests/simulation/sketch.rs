//! A short session for the diff's rules, each line called by a name

use crate::simulation::frozen::Frozen;

pub fn sketch(names: &[&str]) -> Vec<Frozen> {
	names
		.iter()
		.map(|name| {
			let (id, kind, text) = match *name {
				"scope" => ("scope", "comment", "# run · the recorded intent"),
				"intent" => ("intent", "output", "console › intent     sonnet"),
				"other intent" => ("intent", "output", "console › intent     opus"),
				"success intent" => ("intent", "success", "console › intent     opus"),
				"gate" => ("gateOpened", "success", "console › gate       $11.40"),
				"other gate" => {
					("gateOpened", "success", "console › gate       $40.80")
				}
				"output gate" => {
					("gateOpened", "output", "console › gate       $11.40")
				}
				"early runtime" => ("runtimeReady", "output", "runtime    ready"),
				"next" => ("next", "comment", "# next: the merge stays the owner's"),
				other => panic!("no line is named {other}"),
			};

			Frozen {
				id: String::from(id),
				kind: String::from(kind),
				text: String::from(text),
			}
		})
		.collect()
}
