use std::collections::HashMap;
use super::WordCounter;

#[derive(Default, Debug)]
pub struct NestedWordCounter {
    counter: HashMap<String, WordCounter>
}

impl NestedWordCounter {
    pub fn new() -> Self {
        Self {
            counter: HashMap::new()
        }
    }

    pub fn increment(&mut self, outer_key: &str, inner_key: &str) {
        self.counter.entry(outer_key.into())
            .or_insert(WordCounter::new())
            .increment(inner_key);
    }

    pub fn len(&self) -> usize {
        self.counter.len()
    }

    pub fn iter(&self) -> impl Iterator<Item=(&String, &WordCounter)> {
        self.counter.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item=(String, WordCounter)> {
        self.counter.into_iter()
    }

    pub fn keys(&self) -> impl Iterator<Item=&String> {
        self.counter.keys()
    }

    pub fn values(&self) -> impl Iterator<Item=&WordCounter> {
        self.counter.values()
    }
}

