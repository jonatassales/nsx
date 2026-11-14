//! A judge behind a console without the team's Anthropic key
//!
//! Whatever is asked, the console answers `model-key-missing`.

mod ask;

pub struct KeylessJudge;
