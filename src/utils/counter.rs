use std::collections::{HashMap, hash_map};

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

    pub fn iter(&self) -> StringCounterIter {
        StringCounterIter { iter: self.counter.iter() }
    }

    pub fn keys(&self) -> impl Iterator<Item=&String> {
        self.counter.keys()
    }

    pub fn values(&self) -> impl Iterator<Item=&usize> {
        self.counter.values()
    }
}

pub struct StringCounterIter<'a> {
    iter: hash_map::Iter<'a, String, usize>
}

impl<'a> Iterator for StringCounterIter<'a> {
    type Item = (&'a String, &'a usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl IntoIterator for StringCounter {
    type Item = (String, usize);
    type IntoIter = hash_map::IntoIter<String, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.counter.into_iter()
    }
}