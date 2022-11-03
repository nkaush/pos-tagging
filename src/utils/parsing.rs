const TAG_DELIMITER: char = '=';

pub fn extract_word_and_tag(sentence: &str) -> Vec<(&str, &str)> {
    sentence
        .split(' ')
        .map(|pair| pair.split_once(TAG_DELIMITER))
        .map(Option::unwrap)
        .collect()
}