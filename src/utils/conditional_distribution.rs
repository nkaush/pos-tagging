use super::{StringFrequencyDistribution, NestedStringCounter};
use super::{ALPHA, LIKELIHOOD_LOG_BASE};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ConditionalStringFrequencyDistribution {
    distribution: HashMap<String, StringFrequencyDistribution>
}

impl ConditionalStringFrequencyDistribution {
    pub fn with_default_smoothing(counter: NestedStringCounter) -> Self {
        let distribution = counter.into_iter()
            .map(|(tag, counter)| (tag, StringFrequencyDistribution::with_default_smoothing(counter)))
            .collect();

        Self { distribution }
    }

    pub fn with_conditional_smoothing(counter: NestedStringCounter, smoothing: StringFrequencyDistribution) -> Self {
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
        self.distribution.values()
            .map(|d| d.contains_key(inner_key))
            .any(|b| b)
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
