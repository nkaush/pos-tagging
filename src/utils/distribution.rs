use std::collections::{HashMap, hash_map};
use serde::{Deserialize, Serialize};
use super::StringCounter;

pub(in crate::utils) const ALPHA: f64 = 1e-5;
pub(in crate::utils) const LIKELIHOOD_LOG_BASE: f64 = std::f64::consts::E;

#[derive(Debug, Serialize, Deserialize)]
pub struct StringFrequencyDistribution {
    distribution: HashMap<String, f64>,
    smoothed_default: f64
}

impl StringFrequencyDistribution {
    pub fn with_default_smoothing(counter: StringCounter) -> Self {
        Self::with_smoothing(counter, ALPHA)
    }

    pub fn with_smoothing(counter: StringCounter, smoothing_scale: f64) -> Self {
        let n: f64 = counter.values().sum::<usize>() as f64;
        let v: f64 = counter.len() as f64;
        let denominator = n + (smoothing_scale * (v + 1f64));

        let entry_to_prob = 
            |(key, count): (String, usize)| (key, ((count as f64 + smoothing_scale) / denominator).ln());

        let smoothed_default = (smoothing_scale / denominator).ln();
        Self {
            distribution: HashMap::from_iter(counter.into_iter().map(entry_to_prob)),
            smoothed_default
        }
    }

    pub fn get_likelihood(&self, key: &str) -> f64 {
        *self.distribution
            .get(key)
            .unwrap_or(&self.smoothed_default)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.distribution.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item=(&String, &f64)> {
        self.distribution.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item=&String> {
        self.distribution.keys()
    }

    pub fn values(&self) -> impl Iterator<Item=&f64> {
        self.distribution.values()
    }
}

impl IntoIterator for StringFrequencyDistribution {
    type Item = (String, f64);
    type IntoIter = hash_map::IntoIter<String, f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.distribution.into_iter()
    }
}