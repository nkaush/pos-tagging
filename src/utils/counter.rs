use std::collections::{HashMap, hash_map};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StringCounter {
    counter: HashMap<String, usize>
}

impl StringCounter {
    pub fn new() -> Self {
        Self {
            counter: HashMap::new()
        }
    }

    pub fn increment(&mut self, key: &str) {
        self.counter
            .entry(key.into())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    pub fn extend(&mut self, other: StringCounter) {
        self.counter.extend(other.into_iter())
    }

    pub fn len(&self) -> usize {
        self.counter.len()
    }

    pub fn iter(&self) -> impl Iterator<Item=(&String, &usize)> {
        self.counter.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item=&String> {
        self.counter.keys()
    }

    pub fn values(&self) -> impl Iterator<Item=&usize> {
        self.counter.values()
    }
}

impl IntoIterator for StringCounter {
    type Item = (String, usize);
    type IntoIter = hash_map::IntoIter<String, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.counter.into_iter()
    }
}