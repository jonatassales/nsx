//! The embedder
//!
//! What turns a text into a vector. The plan embeds the ticket's intent, and
//! every finished run embeds the intent of its episode. It runs on the
//! machine, it is deterministic and it makes no paid call, so embedding costs
//! nothing. The index records which embedder built it, by its id and its
//! dimensions, so that an index built by another embedder is noticed.
//!
//! Definition only: the `Embedder` type, with its id and its dimensions
//! beside it. It is a plain type, not a trait: the tests run it for real.
//! Its function is the file in `embedder/`.
//!
//! ## Rules
//!
//! - It makes no paid call: the plan step pays for nothing.
//! - A change to how it embeds is a new id, so every index is rebuilt.

pub mod embed;

pub const ID: &str = "local-hashed-tfidf-1";

pub const DIMENSIONS: usize = 1024;

#[derive(Debug, Clone, Copy, Default)]
pub struct Embedder;
