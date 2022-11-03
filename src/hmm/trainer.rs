use crate::hmm::hapax_patterns::get_matching_artificial_tag;
use crate::nlp::extract_word_and_tag;
use crate::POSTaggingHMM;
use crate::utils::*;

use std::io::{self, BufReader, BufRead};
use std::path::PathBuf;
use std::error::Error;
use std::fs::File;

#[derive(Debug)]
pub struct POSTaggingHMMTrainer {
    was_trained: bool,
    initial_tag_counts: StringCounter,
    tag_emission_counts: NestedStringCounter,
    tag_transition_counts: NestedStringCounter
}

impl POSTaggingHMMTrainer {
    pub fn new() -> Self {
        Self {
            was_trained: false,
            initial_tag_counts: StringCounter::new(),
            tag_emission_counts: NestedStringCounter::new(),
            tag_transition_counts: NestedStringCounter::new()
        }
    }

    pub fn train(mut self, data_file: PathBuf) -> Result<Self, io::Error> {
        let f = File::open(data_file)?;
        let rdr = BufReader::new(f);

        self.was_trained = true;

        for line in rdr.lines() {
            let sentence = line?;
            let tagged_line = extract_word_and_tag(&sentence);

            let mut iter = tagged_line.into_iter();
            let (w0, t0) = iter.next().unwrap();
            
            self.initial_tag_counts.increment(&t0);
            self.tag_emission_counts.increment(&t0, &w0);

            let mut previous_tag = t0;
            for (word, tag) in iter {
                self.tag_emission_counts.increment(&tag, &word);
                self.tag_transition_counts.increment(&previous_tag, &tag);
                
                previous_tag = tag;
            }
        }

        Ok(self)
    }

    pub fn finalize(mut self) -> Result<POSTaggingHMM, Box<dyn Error>> {
        if !self.was_trained {
            Err("Model has not yet been trained")?
        }

        let mut hapax_counts = StringCounter::new();
        let mut artificial_word_counts = NestedStringCounter::new();
        
        for (tag, word_counts) in self.tag_emission_counts.iter() {
            for (word, count) in word_counts.iter() {
                if count == &1 {
                    hapax_counts.increment(tag);

                    if let Some(aw) = get_matching_artificial_tag(word) {
                        artificial_word_counts.increment(tag, aw);
                    }
                }
            }
        }

        self.tag_emission_counts.extend(artificial_word_counts);
        let hapax_distribution = 
            StringFrequencyDistribution::with_default_smoothing(hapax_counts);

        println!("{:?}", self.tag_emission_counts);
        let emission_distribution = 
            ConditionalStringFrequencyDistribution::with_conditional_smoothing(
                self.tag_emission_counts, 
                hapax_distribution
            );

        let transition_distribution = 
            ConditionalStringFrequencyDistribution::with_default_smoothing(
                self.tag_transition_counts
            );

        let initial_tag_distribution =
            StringFrequencyDistribution::with_default_smoothing(
                self.initial_tag_counts
            );

        Ok(POSTaggingHMM::new(
            initial_tag_distribution,
            emission_distribution,
            transition_distribution
        ))
    }
}