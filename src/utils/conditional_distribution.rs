use super::{StringFrequencyDistribution, ConditionalStringCounter, ALPHA};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(in crate::utils) const LIKELIHOOD_LOG_BASE: f64 = std::f64::consts::E;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionalStringFrequencyDistribution {
    distribution: HashMap<String, StringFrequencyDistribution>
}

impl ConditionalStringFrequencyDistribution {
    pub fn with_default_smoothing(counter: ConditionalStringCounter) -> Self {
        let distribution = counter.into_iter()
            .map(|(tag, c)| (tag, StringFrequencyDistribution::with_default_smoothing(c)))
            .collect();

        Self { distribution }
    }

    pub fn with_conditional_smoothing(counter: ConditionalStringCounter, smoothing: StringFrequencyDistribution) -> Self {
        let distribution = counter.into_iter()
            .map(|(tag, counter)| {
                let alpha = LIKELIHOOD_LOG_BASE.powf(smoothing.get_likelihood(&tag)) * ALPHA;
                (tag, StringFrequencyDistribution::with_smoothing(counter, alpha))
            })
            .collect();

        Self { distribution }
    }

    pub fn get_likelihood(&self, outer_key: &str, inner_key: &str) -> Option<f64> {
        self.distribution
            .get(outer_key)
            .map(|s| s.get_likelihood(inner_key))
    }

    pub fn inner_key_exists(&self, inner_key: &str) -> bool {
        self.distribution
            .values()
            .any(|d| d.contains_key(inner_key))
    } 

    pub fn iter(&self) -> impl Iterator<Item=(&String, &StringFrequencyDistribution)> {
        self.distribution.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item=(String, StringFrequencyDistribution)> {
        self.distribution.into_iter()
    }

    pub fn keys(&self) -> impl Iterator<Item=&String> {
        self.distribution.keys()
    }

    pub fn values(&self) -> impl Iterator<Item=&StringFrequencyDistribution> {
        self.distribution.values()
    }
}
