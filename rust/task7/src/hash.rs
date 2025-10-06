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
    entries: Vec<Vec<Entry<V>>>,
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
            if el.is_empty() {
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
            entries: vec![Vec::new(); cap],
        }
    }

    pub fn insert(&mut self, key: InputValue, value: V) -> &mut Self {
        let hashed = match key {
            InputValue::Text(ref text) => hash_by_string(text.as_str(), self.entries.len() as u32),
            InputValue::Number(num) => {
                hash_by_multiplication(num as u32, self.entries.len() as u32)
            }
        } as usize;

        let slot = self.entries.get_mut(hashed).unwrap();

        for entry in slot.iter_mut() {
            if entry.key == key {
                entry.value = value;
                return self;
            }
        }

        slot.push(Entry {
            key: key.clone(),
            key_type: key.into(),
            value,
        });

        self
    }

    pub fn remove(&mut self, key: InputValue) -> &mut Self {
        let hashed = match key {
            InputValue::Text(ref text) => hash_by_string(text.as_str(), self.entries.len() as u32),
            InputValue::Number(num) => {
                hash_by_multiplication(num as u32, self.entries.len() as u32)
            }
        } as usize;

        let slot = self.entries.get_mut(hashed).unwrap();

        for i in 0..slot.len() {
            if slot[i].key == key {
                slot.remove(i);
                break;
            }
        }

        self
    }

    pub fn find(&self, key: InputValue) -> Option<&V> {
        let hashed = match key {
            InputValue::Text(ref text) => hash_by_string(text.as_str(), self.entries.len() as u32),
            InputValue::Number(num) => {
                hash_by_multiplication(num as u32, self.entries.len() as u32)
            }
        } as usize;

        let slot = self.entries.get(hashed).unwrap();

        for i in 0..slot.len() {
            if slot[i].key == key {
                return Some(&slot[i].value);
            }
        }

        None
    }
}
