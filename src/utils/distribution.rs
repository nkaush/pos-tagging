use std::collections::HashMap;
use super::WordCounter;

const ALPHA: f64 = 1e-5;

#[derive(Debug)]
pub struct StringFrequencyDistribution {
    distribution: HashMap<String, f64>,
    smoothed_default: f64
}

impl StringFrequencyDistribution {
    pub fn with_default_smoothing(counter: WordCounter) -> Self {
        Self::with_smoothing(counter, ALPHA)
    }

    pub fn with_smoothing(counter: WordCounter, smoothing_scale: f64) -> Self {
        let n: f64 = counter.values().sum::<usize>() as f64;
        let v: f64 = counter.len() as f64;
        let denominator = n + (smoothing_scale * (v + 1f64));

        let entry_to_prob = 
            |(key, count): (String, usize)| (key, ((count as f64 + smoothing_scale) / denominator).log10());

        Self {
            distribution: HashMap::from_iter(counter.into_iter().map(entry_to_prob)),
            smoothed_default: (smoothing_scale / denominator).log10()
        }
    }

    pub fn get_likelihood(&self, key: &String) -> f64 {
        *self.distribution
            .get(key)
            .unwrap_or(&self.smoothed_default)
    }
}