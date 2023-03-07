use crate::{StringFrequencyDistribution, ConditionalStringFrequencyDistribution};
use crate::nlp::{get_matching_artificial_tag, TaggedWord, END_TAG};
use std::{
    fs::{File, OpenOptions}, io::{Write, Read}, error::Error,
    collections::HashMap, path::PathBuf
};
use serde::{Deserialize, Serialize};

const MODEL_FILE_HEADER: [u8; 4] = *b"VHMM";

#[derive(Debug, Deserialize, Serialize)]
pub struct POSTaggingHMM {
    initial_tag_distribution: StringFrequencyDistribution,
    emission_distribution: ConditionalStringFrequencyDistribution,
    transition_distribution: ConditionalStringFrequencyDistribution,
    tag_set: Vec<String>,
    tag_indices: HashMap<String, usize>
}

impl POSTaggingHMM {
    pub(in crate::hmm) fn new(
        initial_tag_distribution: StringFrequencyDistribution,
        emission_distribution: ConditionalStringFrequencyDistribution,
        transition_distribution: ConditionalStringFrequencyDistribution
    ) -> Self {
        let tag_set: Vec<String> = transition_distribution.keys()
            .cloned()
            .collect();
        let tag_indices = tag_set.iter()
            .cloned()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect();

        Self {
            initial_tag_distribution,
            emission_distribution,
            transition_distribution,
            tag_set,
            tag_indices
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut header: [u8; 4] = [0; 4];
        file.read_exact(&mut header)?;
        if header != MODEL_FILE_HEADER {
            Err("Unknown file structure")?
        }

        let mut model_bytes = Vec::new();
        file.read_to_end(&mut model_bytes)?;

        Ok(bincode::deserialize(&model_bytes)?)
    }

    pub fn save(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?;
        
        file.write_all(&MODEL_FILE_HEADER)?;
        file.write_all(bincode::serialize(&self)?.as_ref())?;

        Ok(())
    }

    pub fn predict(&self, mut sentence: Vec<String>) -> Vec<TaggedWord> {
        sentence = sentence
            .iter()
            .map(|s| s.to_ascii_lowercase())
            .collect();
        sentence.push(END_TAG.into());

        let mut b = vec![vec![""; sentence.len()]; self.tag_set.len()];

        let mut cv: Vec<f64> = vec![0.0; self.tag_set.len()];
        let mut pv: Vec<f64> = self.tag_set
            .iter()
            .map(|tag| {
                self.initial_tag_distribution.get_likelihood(tag) 
                    + self.emission_distribution.get_likelihood(tag, &sentence[0]).unwrap()
            })
            .collect();

        for (time, word) in sentence.iter().enumerate().skip(1) {
            let is_unseen = !self.emission_distribution.inner_key_exists(word);
            let mut emission_word = word.as_str();
            if is_unseen {
                if let Some(artificial_tag) = get_matching_artificial_tag(word) {
                    emission_word = artificial_tag;
                } 
            }

            for (cti, curr_tag) in self.tag_set.iter().enumerate() {
                let emission = self.emission_distribution
                    .get_likelihood(curr_tag, emission_word)
                    .unwrap();

                let (best_score, best_tag): (f64, &str) = self.tag_set
                    .iter()
                    .enumerate()
                    .map(|(pti, prev_tag)| {
                        let transition = self.transition_distribution
                            .get_likelihood(prev_tag, &curr_tag)
                            .unwrap();
                        (pv[pti] + emission + transition, prev_tag.as_str())
                    })
                    .max_by(|(s1, _), (s2, _)| s1.total_cmp(s2))
                    .unwrap();

                b[cti][time] = best_tag;
                cv[cti] = best_score;
            }

            pv = cv;
            cv = vec![0.0; self.tag_set.len()];
        }

        let predicted_tags = self.backtrack_trellis(b, pv, sentence.len());

        sentence.into_iter().zip(predicted_tags).collect()
    }

    fn backtrack_trellis(&self, potential_tags: Vec<Vec<&str>>, final_likelihoods: Vec<f64>, sentence_len: usize) -> Vec<String> {
        let (_, best_tag): (f64, &str) = final_likelihoods.into_iter()
            .zip(potential_tags.iter())
            .map(|(vi, bi)| {
                (vi, bi[sentence_len - 1])
            })
            .max_by(|(s1, _), (s2, _)| s1.total_cmp(s2))
            .unwrap();

        let mut previous_tag = best_tag;
        let mut prev_tag_idx = self.tag_indices.get(previous_tag);
        let mut predicted_tags = vec![END_TAG.to_string(); sentence_len - 1];
        
        for (time, pred_tag) in predicted_tags.iter_mut().enumerate().rev() {
            *pred_tag = previous_tag.to_string();

            if prev_tag_idx.is_none() {
                break;
            }

            previous_tag = potential_tags[*prev_tag_idx.unwrap()][time];
            prev_tag_idx = self.tag_indices.get(previous_tag);
        }

        predicted_tags
    }
}