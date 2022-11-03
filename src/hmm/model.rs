use crate::{StringFrequencyDistribution, ConditionalStringFrequencyDistribution};
use std::collections::HashMap;
use crate::nlp::TaggedWord;

use super::hapax_patterns;

#[derive(Debug)]
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

    pub fn predict(&self, sentence: Vec<String>) -> Vec<TaggedWord> {
        // len(tagset) x len(sentence) arrays
        let mut v = vec![vec![0.0; sentence.len()]; self.tag_set.len()];
        let mut b = vec![vec![""; sentence.len()]; self.tag_set.len()];

        for (i, tag) in self.tag_set.iter().enumerate() {
            v[i][0] = self.initial_tag_distribution.get_likelihood(tag);
            v[i][0] += self.emission_distribution.get_likelihood(tag, &sentence[0]).unwrap();
        }

        for (time, word) in sentence.iter().enumerate().skip(1) {
            let is_unseen = self.emission_distribution.inner_key_exists(word);
            let artificial_tag = hapax_patterns::get_matching_artificial_tag(word);

            for (cti, curr_tag) in self.tag_set.iter().enumerate() {
                let emission = if is_unseen && artificial_tag.is_some() {
                    self.emission_distribution
                        .get_likelihood(curr_tag, artificial_tag.unwrap())
                        .unwrap()
                } else {
                    self.emission_distribution
                        .get_likelihood(curr_tag, word)
                        .unwrap()
                };

                let (best_score, best_tag): (f64, &str) = self.tag_set
                    .iter()
                    .enumerate()
                    .map(|(pti, prev_tag)| {
                        let transition = self.transition_distribution
                            .get_likelihood(prev_tag, &curr_tag)
                            .unwrap();
    
                        (v[pti][time - 1] + emission + transition, prev_tag.as_str())
                    })
                    .max_by(|(s1, _), (s2, _)| s1.total_cmp(s2))
                    .unwrap();

                b[cti][time] = best_tag;
                v[cti][time] = best_score;
            }
        }

        let (_, best_tag): (f64, &str) = v.iter()
            .zip(b.iter())
            .map(|(vi, bi)| {
                (vi[sentence.len() - 1], bi[sentence.len() - 1])
            })
            .max_by(|(s1, _), (s2, _)| s1.total_cmp(s2))
            .unwrap();

        let mut predicted_tags = vec!["END".to_string(); sentence.len()];
        let mut previous_tag = best_tag;
        let mut prev_tag_idx = self.tag_indices.get(previous_tag);
        
        for (time, pred_tag) in predicted_tags.iter_mut().enumerate().rev().skip(1) {
            *pred_tag = previous_tag.to_string();

            if prev_tag_idx.is_none() {
                break;
            }

            previous_tag = b[*prev_tag_idx.unwrap()][time];
            prev_tag_idx = self.tag_indices.get(previous_tag);
        }

        sentence.into_iter().zip(predicted_tags).collect()
    }
}