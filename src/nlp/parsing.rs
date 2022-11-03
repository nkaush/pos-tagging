const SENTENCE_DELIMITER: char = ' ';
const TAG_DELIMITER: char = '=';
const DELIMITER_REPLACEMENT: &str = "/";

pub fn extract_word_and_tag(sentence: &str) -> Vec<(String, String)> {
    let mut out: Vec<_> = sentence
        .split(SENTENCE_DELIMITER)
        .into_iter()
        .map(|w| (w, w.matches(TAG_DELIMITER).count()))
        .map(|(w, _)| {
            let c: Vec<&str> = w.split(TAG_DELIMITER).collect();
            (c[..c.len() - 1].join(DELIMITER_REPLACEMENT).to_ascii_lowercase(), c[c.len() - 1].to_string())
        })
        .collect();

    out.push(("END".to_string(), "END".to_string()));
    out
}
