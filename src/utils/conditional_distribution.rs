use super::{StringFrequencyDistribution, NestedStringCounter};
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
                let alpha = smoothing.get_likelihood(&tag);
                (tag, StringFrequencyDistribution::with_smoothing(counter, alpha))
            })
            .collect();

        Self { distribution }
    }
}