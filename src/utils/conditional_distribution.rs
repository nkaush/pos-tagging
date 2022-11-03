use super::{StringFrequencyDistribution, NestedWordCounter};
use std::collections::HashMap;

pub struct ConditionalWordFrequencyDistribution {
    distribution: HashMap<String, StringFrequencyDistribution>
}

impl ConditionalWordFrequencyDistribution {
    pub fn with_default_smoothing(counter: NestedWordCounter) -> Self {
        let distribution = counter.into_iter()
            .map(|(tag, counter)| (tag, StringFrequencyDistribution::with_default_smoothing(counter)))
            .collect();

        Self { distribution }
    }

    pub fn with_conditional_smoothing(counter: NestedWordCounter, smoothing: StringFrequencyDistribution) -> Self {
        let distribution = counter.into_iter()
            .map(|(tag, counter)| {
                let alpha = smoothing.get_likelihood(&tag);
                (tag, StringFrequencyDistribution::with_smoothing(counter, alpha))
            })
            .collect();

        Self { distribution }
    }
}