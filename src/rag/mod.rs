//! The RAG over the project's episodes
//!
//! Every finished run leaves one episode under `.langdrift/`. To suggest a
//! model and an effort, the plan embeds the ticket's intent into one query
//! vector, takes the six most similar episodes from the project's index, keeps
//! those at or above 0.55 and cites them.
//!
//! - `Embedder`: turning a text into a vector, on the machine
//! - `Index`: the index of episodes, one `Entry` and one stored vector each
//! - `Candidate`: an episode and its similarity
//! - `Threshold`: how similar is similar enough
//! - `Episode`: one past run, with each `Part` it ran, its `Resource` and
//!   what it `Measured`
//! - `Retrieval`: the search, what it kept and its six lines
//! - `Error`: what the retrieval can refuse
//!
//! ## Rules
//!
//! - Similarity alone ranks and keeps the candidates: no other score exists.
//! - History sizes nothing: no component's share comes from an episode.
//! - Each candidate's reason, the sizing basis and the suggestion are the
//!   judge's.

pub mod candidate;
pub mod embedder;
pub mod entry;
pub mod episode;
pub mod error;
pub mod index;
pub mod measured;
pub mod part;
pub mod resource;
pub mod retrieval;
pub mod threshold;
