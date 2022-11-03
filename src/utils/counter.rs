use std::collections::HashMap;

#[derive(Default, Debug)]
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

    pub fn into_iter(self) -> impl Iterator<Item=(String, usize)> {
        self.counter.into_iter()
    }

    pub fn keys(&self) -> impl Iterator<Item=&String> {
        self.counter.keys()
    }

    pub fn values(&self) -> impl Iterator<Item=&usize> {
        self.counter.values()
    }
}

