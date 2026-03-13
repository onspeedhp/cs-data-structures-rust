use core::str;
use std::usize;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    capacity: usize,
    entries: Vec<(K, V)>,
}

impl<K: PartialEq, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Vec::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let index: Option<usize> = self.entries.iter().position(|entry| &entry.0 == key);

        if let Some(i) = index {
            let item = self.entries.remove(i);
            self.entries.insert(0, item);
            Some(&self.entries[0].1)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        let index: Option<usize> = self.entries.iter().position(|entry| &entry.0 == &key);

        if let Some(i) = index {
            let item = self.entries.remove(i);
            self.entries.insert(0, item);
        } else {
            self.entries.insert(0, (key, value));
        }

        if self.entries.len() > self.capacity {
            self.entries.pop();
        }
    }

    pub fn print(&self, str: &str)
    where
        K: std::fmt::Debug,
        V: std::fmt::Debug,
    {
        println!("{}:", str);
        for entry in &self.entries {
            println!("{:?}: {:?}", entry.0, entry.1);
        }
    }
}
