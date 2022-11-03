use crate::utils::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

// type Counter = HashMap<String, usize>;
type Distribution = HashMap<String, f64>;


const ALPHA: f64 = 1e-5;
const DEFAULT_ALPHA_SCALAR: f64 = 1e-5;

#[derive(Default)]
pub struct PosTaggerHiddenMarkovModel {
    starting_tag_counts: Counter,
    hapax_word_counts: Counter,
    hapax_artificial_tag_counts: HashMap<String, Counter>,
    transition_counts: HashMap<String, Counter>,
    emission_counts: HashMap<String, Counter>,

    hapax_probabilities: Distribution,
    starting_tag_probabilities: Distribution,
    emission_probabilities: HashMap<String, Distribution>,
    transition_probabilities: HashMap<String, Distribution>,
}

fn get_matching_artificial_tag(word: &str) -> Option<String> {
    for (suffix, tag) in ARTIFICIAL_TAG_SUFFIXES {
        if word.ends_with(suffix) {
            return Some(tag.into());
        }
    }

    for (suffix, tag) in ARTIFICIAL_TAG_SUFFIXES {
        if word.starts_with(suffix) {
            return Some(tag.into());
        }
    }

    let num_digits = word.chars().filter(char::is_ascii_digit).count();

    if num_digits > word.len() / 2 {
        return Some("IS-A-NUMBER".into());
    } else if word.chars().any(|c| c == '-') {
        return Some("HAS-MANY-DASHES".into());
    }

    None
}

fn increment_nested_counter(map: &mut HashMap<String, Counter>, outer: &str, inner: &str) {
    map.entry(outer.into())
        .and_modify(|m| {
            m.entry(inner.into()).and_modify(|e| *e += 1).or_insert(1);
        })
        .or_insert_with(|| HashMap::from_iter([(inner.into(), 1)]));
}

fn increment_counter(map: &mut Counter, key: &str) {
    map.entry(key.into()).and_modify(|e| *e += 1).or_insert(1);
}

impl PosTaggerHiddenMarkovModel {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn train(&mut self, data_file: PathBuf) -> Result<(), io::Error> {
        let f = File::open(data_file)?;
        let rdr = BufReader::new(f);

        for line in rdr.lines() {
            let sentence = line?;
            let tagged_line = extract_word_and_tag(&sentence);
            let (w0, t0) = tagged_line[0];

            increment_counter(&mut self.starting_tag_counts, t0);
            increment_nested_counter(&mut self.emission_counts, t0, w0);

            let mut previous_tag = t0;
            for (word, tag) in tagged_line.into_iter().skip(1) {
                increment_nested_counter(&mut self.transition_counts, previous_tag, tag);
                increment_nested_counter(&mut self.emission_counts, tag, word);

                previous_tag = tag;
            }
        }

        Ok(())
    }

    fn aggregate_hapax_distrubtion(&mut self) {
        let mut hapax_total: usize = 0;

        for (tag, counts_by_tag) in self.emission_counts.iter() {
            for (word, count) in counts_by_tag.iter() {
                if count == &1 {
                    increment_counter(&mut self.hapax_word_counts, tag);
                    if let Some(artificial_tag) = get_matching_artificial_tag(word) {
                        increment_nested_counter(
                            &mut self.hapax_artificial_tag_counts,
                            &artificial_tag,
                            tag,
                        );
                    }
                }
            }

            hapax_total += self.hapax_word_counts.get(tag).unwrap_or(&0);
        }

        let hapax_total: f64 = hapax_total as f64;
        self.hapax_probabilities.extend(
            self.hapax_word_counts
                .iter()
                .map(|(key, value)| (key.to_owned(), *value as f64 / hapax_total)),
        );
        self.hapax_probabilities.insert(String::new(), ALPHA);
    } 

    fn aggregate_emission_distributions(&mut self) {
        for (tag, counts_by_tag) in self.emission_counts.iter() {
            let v = counts_by_tag.len() as f64;
            let n = counts_by_tag.values().map(usize::to_owned).sum::<usize>() as f64;
            let alpha_scalar = self
                .hapax_probabilities
                .get(tag)
                .unwrap_or(&DEFAULT_ALPHA_SCALAR);
            let tag_alpha = ALPHA * alpha_scalar;

            let denominator = n + (tag_alpha * (v + 1f64));
            self.emission_probabilities.insert(
                tag.into(),
                HashMap::from_iter(counts_by_tag.iter().map(|(key, count)| {
                    (
                        key.clone(),
                        ((*count as f64 + tag_alpha) / denominator).log10(),
                    )
                })),
            );

            for (artificial_word, map) in self.hapax_artificial_tag_counts.iter() {
                let total: usize = map.values().sum();
                let total = total as f64;
                
                let p = if total > 0.0 && map.contains_key(tag) {
                    (*map.get(tag).unwrap_or(&0) as f64 / total).log10()
                } else {
                    1e-7_f64.log10()
                };

                self.emission_probabilities.get_mut(tag.into()).unwrap().insert(artificial_word.into(), p);
            }
        }
    }

    fn aggregate(&mut self) {
        self.aggregate_hapax_distrubtion();

        
    }
}
