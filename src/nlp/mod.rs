mod parsing;
mod hapax_patterns;

pub use parsing::*;
pub use hapax_patterns::*;

pub type TaggedWord = (String, String);
pub type TaggedSentence = Vec<TaggedWord>;