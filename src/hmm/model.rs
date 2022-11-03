use crate::{StringFrequencyDistribution, ConditionalStringFrequencyDistribution};

#[derive(Debug)]
pub struct POSTaggingHMM {
    initial_tag_distribution: StringFrequencyDistribution,
    emission_distribution: ConditionalStringFrequencyDistribution,
    transition_distribution: ConditionalStringFrequencyDistribution
}

impl POSTaggingHMM {
    pub(in crate::hmm) fn new(
        initial_tag_distribution: StringFrequencyDistribution,
        emission_distribution: ConditionalStringFrequencyDistribution,
        transition_distribution: ConditionalStringFrequencyDistribution
    ) -> Self {
        Self {
            initial_tag_distribution,
            emission_distribution,
            transition_distribution
        }
    }
}