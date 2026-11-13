//! The kind of a linked document
//!
//! A ticket links four documents, one of each kind. A document's kind comes
//! from its file's suffix and from nothing else: never from a model.
//!
//! Definition only: the `Kind` type: the suffix a file of this kind ends
//! with, the words a sentence names it by, its word on the Ticket card, and
//! the folder its documents sit in when that folder is LangDrift's own.
//! Beside it, the four kinds, in the order the lines and the card name them:
//! the PRD, the design doc, the spec and the guardrail.
//!
//! ## Rules
//!
//! - The PRD, the design doc and the spec are the client's and sit in the
//!   client's artifact folder, so they name no folder of their own. The
//!   guardrail is LangDrift's and sits in `.langdrift/harness/guardrails/`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kind {
	pub suffix: &'static str,
	pub name: &'static str,
	pub word: &'static str,
	pub home: Option<&'static str>,
}

pub const PRD: Kind = Kind {
	suffix: ".prd.md",
	name: "PRD",
	word: "prd",
	home: None,
};

pub const DESIGN: Kind = Kind {
	suffix: ".dd.md",
	name: "design doc",
	word: "designDoc",
	home: None,
};

pub const SPEC: Kind = Kind {
	suffix: ".spec.md",
	name: "spec",
	word: "spec",
	home: None,
};

pub const GUARDRAIL: Kind = Kind {
	suffix: ".gr.md",
	name: "guardrail",
	word: "guardrail",
	home: Some(".langdrift/harness/guardrails"),
};

pub const KINDS: [Kind; 4] = [PRD, DESIGN, SPEC, GUARDRAIL];
