mod parsing;

pub use parsing::*;

// pub struct TaggedWord {
//     pub word: String, 
//     pub pos_tag: String
// }

pub type TaggedWord = (String, String);
pub type TaggedSentence = Vec<TaggedWord>;