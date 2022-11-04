use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::StringCounter;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ConditionalStringCounter {
    counter: HashMap<String, StringCounter>
}

impl ConditionalStringCounter {
    pub fn new() -> Self {
        Self {
            counter: HashMap::new()
        }
    }

    pub fn increment(&mut self, outer_key: &str, inner_key: &str) {
        self.counter.entry(outer_key.into())
            .or_insert(StringCounter::new())
            .increment(inner_key);
    }

    pub fn extend(&mut self, other: ConditionalStringCounter) {
        for (tag, counter) in other.into_iter() {
            self.counter
                .entry(tag)
                .or_insert(StringCounter::new())
                .extend(counter);
        }
    }

    pub fn len(&self) -> usize {
        self.counter.len()
    }

    pub fn iter(&self) -> impl Iterator<Item=(&String, &StringCounter)> {
        self.counter.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item=(String, StringCounter)> {
        self.counter.into_iter()
    }

    pub fn keys(&self) -> impl Iterator<Item=&String> {
        self.counter.keys()
    }

    pub fn values(&self) -> impl Iterator<Item=&StringCounter> {
        self.counter.values()
    }
}
