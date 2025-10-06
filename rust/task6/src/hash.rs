use std::fmt::Display;

use crate::input::{InputType, InputValue};

static A: f64 = 0.6180339887;

pub fn hash_by_division(key: u32, table_size: u32) -> u32 {
    (key % table_size) as u32
}

pub fn hash_by_multiplication(key: u32, table_size: u32) -> u32 {
    let fract = (key as f64 * A) % 1.0;
    (table_size as f64 * fract).floor() as u32
}

pub fn hash_by_string(key: &str, table_size: u32) -> u32 {
    let mut hash: u32 = 0;
    for b in key.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(b as u32);
    }
    (hash as u32) % table_size
}

#[derive(Clone)]
pub struct Entry<V> {
    key: InputValue,
    key_type: InputType,
    value: V,
}

#[derive(Clone)]
pub struct HashTable<V> {
    entries: Vec<Option<Entry<V>>>,
}

impl<V> Display for HashTable<V>
where
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.entries.is_empty() {
            writeln!(f)?;
            write!(f, "EMPTY")?;
            return Ok(());
        }

        self.entries.iter().enumerate().for_each(|(i, el)| {
            if el.is_none() {
                return;
            }

            writeln!(f)
                .map_err(|e| {
                    println!("{}", e);
                    std::process::exit(1);
                })
                .ok();

            writeln!(f, "hash {}", i)
                .map_err(|e| {
                    println!("{}", e);
                    std::process::exit(1);
                })
                .ok();
            el.iter().for_each(|entry| {
                writeln!(f, "key - {}, value - {}", entry.key, entry.value)
                    .map_err(|e| {
                        println!("{}", e);
                        std::process::exit(1);
                    })
                    .ok();
            });
            writeln!(f, "----------------------------")
                .map_err(|e| {
                    println!("{}", e);
                    std::process::exit(1);
                })
                .ok();
        });
        Ok(())
    }
}

impl<V> HashTable<V>
where
    V: Clone,
{
    pub fn new(cap: usize) -> Self {
        HashTable {
            entries: vec![None; cap],
        }
    }

    pub fn insert(&mut self, key: InputValue, value: V) -> &mut Self {
        let hashed = match &key {
            InputValue::Text(text) => hash_by_string(text, self.entries.len() as u32) as usize,
            InputValue::Number(num) => {
                hash_by_multiplication(*num as u32, self.entries.len() as u32) as usize
            }
        };

        if let Some(slot) = self.entries.get_mut(hashed) {
            match slot {
                Some(entry) if entry.key == key => {
                    entry.value = value;
                }
                None => {
                    *slot = Some(Entry {
                        key: key.clone(),
                        key_type: key.clone().into(),
                        value,
                    });
                }
                Some(_) => {
                    *slot = Some(Entry {
                        key: key.clone(),
                        key_type: key.clone().into(),
                        value,
                    });
                }
            }
        }

        self
    }

    pub fn remove(&mut self, key: InputValue) -> &mut Self {
        let hashed = match &key {
            InputValue::Text(text) => hash_by_string(text, self.entries.len() as u32) as usize,
            InputValue::Number(num) => {
                hash_by_multiplication(*num as u32, self.entries.len() as u32) as usize
            }
        };

        let mut i = hashed;
        loop {
            match &self.entries[i] {
                Some(entry) if entry.key == key => {
                    self.entries[i] = None;

                    let mut j = (i + 1) % self.entries.len();
                    while let Some(entry) = self.entries[j].take() {
                        let new_hashed = match &entry.key {
                            InputValue::Text(text) => {
                                hash_by_string(text, self.entries.len() as u32) as usize
                            }
                            InputValue::Number(num) => {
                                hash_by_multiplication(*num as u32, self.entries.len() as u32)
                                    as usize
                            }
                        };

                        let mut k = new_hashed;
                        while self.entries[k].is_some() {
                            k = (k + 1) % self.entries.len();
                        }
                        self.entries[k] = Some(entry);

                        j = (j + 1) % self.entries.len();
                    }

                    return self;
                }
                None => return self,
                _ => {
                    i = (i + 1) % self.entries.len();
                    if i == hashed {
                        return self;
                    }
                }
            }
        }
    }

    pub fn find(&self, key: InputValue) -> Option<&V> {
        if self.entries.is_empty() {
            return None;
        }

        let hashed = match key {
            InputValue::Text(ref text) => {
                hash_by_string(text.as_str(), self.entries.len() as u32) as usize
            }
            InputValue::Number(num) => {
                hash_by_multiplication(num as u32, self.entries.len() as u32) as usize
            }
        };

        let mut i = hashed;
        loop {
            match &self.entries[i] {
                Some(entry) if entry.key == key => return Some(&entry.value),
                None => return None,
                _ => {
                    i = (i + 1) % self.entries.len();
                    if i == hashed {
                        return None;
                    }
                }
            }
        }
    }
}
