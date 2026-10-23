use std::collections::HashMap;
use std::ptr;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

type Link<K, V> = *mut Node<K, V>;

struct Node<K, V> {
    key: Option<K>,
    value: Option<V>,
    forward: Vec<Link<K, V>>,
    level: usize,
}

impl<K, V> Display for Node<K, V>
where
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match (&self.key, &self.value) {
            (Some(key), Some(value)) => write!(f, "{}({})", key, value),
            _ => write!(f, "head"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum LevelMode {
    Deterministic,
    Random,
}

impl FromStr for LevelMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "deterministic" | "d" => Ok(Self::Deterministic),
            "random" | "r" => Ok(Self::Random),
            other => Err(format!("unknown mode '{other}'")),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MaxLevelMode {
    Constant(usize),
    Unlimited,
    Function(fn(f64, usize) -> usize),
}

impl FromStr for MaxLevelMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.eq_ignore_ascii_case("unlimited") || s.eq_ignore_ascii_case("u") {
            return Ok(Self::Unlimited);
        }

        let lower = s.to_ascii_lowercase();
        let inner = lower
            .strip_prefix("constant(")
            .or_else(|| lower.strip_prefix("c("))
            .and_then(|rest| rest.strip_suffix(')'));

        if let Some(inner) = inner {
            let n = inner
                .trim()
                .parse::<usize>()
                .map_err(|e| format!("invalid number in '{s}': {e}"))?;
            return Ok(Self::Constant(n));
        }

        if s.eq_ignore_ascii_case("log") || s.eq_ignore_ascii_case("l") {
            return Ok(Self::Function(crate::max_level_mode_log));
        }

        Err(format!("expected 'constant(<number>)', 'unlimited', or 'log', got '{s}'"))
    }
}

pub struct SkipList<K, V> {
    head: Link<K, V>,

    length: usize,
    current_level: usize,
    insert_counter: usize,

    probability: f64,
    rng_state: u32,

    level_mode: LevelMode,
    max_level_mode: MaxLevelMode,
}

impl<K, V> SkipList<K, V> {
    pub fn new(level_mode: LevelMode, max_level_mode: MaxLevelMode, probability: f64) -> Self {
        let initial_level = match max_level_mode {
            MaxLevelMode::Constant(n) => n.max(1),
            MaxLevelMode::Unlimited => 1,
            MaxLevelMode::Function(f) => (f)(probability, 1).max(1),
        };

        let head = Box::into_raw(Box::new(Node {
            key: None,
            value: None,
            forward: vec![ptr::null_mut(); initial_level],
            level: initial_level,
        }));

        Self {
            head,
            length: 0,
            current_level: 1,
            insert_counter: 1,
            probability,
            rng_state: 0x1,
            level_mode,
            max_level_mode,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn head_capacity(&self) -> usize {
        unsafe { (&(*self.head).forward).len() }
    }

    fn ensure_head_capacity(&mut self, level: usize) {
        unsafe {
            let head = &mut *self.head;
            if head.forward.len() < level {
                head.forward.resize(level, ptr::null_mut());
                head.level = level;
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V)
    where
        K: Ord,
    {
        unsafe {
            let mut update: Vec<Link<K, V>> = vec![ptr::null_mut(); self.head_capacity()];
            let mut current = self.head;

            for i in (0..self.current_level).rev() {
                while !(&(*current).forward)[i].is_null()
                    && (*(&(*current).forward)[i]).key.as_ref().unwrap() < &key
                {
                    current = (&(*current).forward)[i];
                }
                update[i] = current;
            }

            let candidate = (&(*current).forward)[0];
            if !candidate.is_null() && (*candidate).key.as_ref().unwrap() == &key {
                (*candidate).value = Some(value);
                return;
            }

            let new_level = self.pick_level();

            self.ensure_head_capacity(new_level);
            if update.len() < new_level {
                update.resize(new_level, ptr::null_mut());
            }

            if new_level > self.current_level {
                for i in self.current_level..new_level {
                    update[i] = self.head;
                }
                self.current_level = new_level;
            }

            let new_node = Box::into_raw(Box::new(Node {
                key: Some(key),
                value: Some(value),
                forward: vec![ptr::null_mut(); new_level],
                level: new_level,
            }));

            for i in 0..new_level {
                (&mut (*new_node).forward)[i] = (&(*update[i]).forward)[i];
                (&mut (*update[i]).forward)[i] = new_node;
            }

            self.length += 1;
        }
    }

    pub fn search(&self, key: &K) -> Option<&V>
    where
        K: Ord,
    {
        unsafe {
            let mut current = self.head;

            for i in (0..self.current_level).rev() {
                while !(&(*current).forward)[i].is_null()
                    && (*(&(*current).forward)[i]).key.as_ref().unwrap() < key
                {
                    current = (&(*current).forward)[i];
                }
            }

            let candidate = (&(*current).forward)[0];
            if !candidate.is_null() && (*candidate).key.as_ref().unwrap() == key {
                (*candidate).value.as_ref()
            } else {
                None
            }
        }
    }

    pub fn delete(&mut self, key: &K) -> Option<V>
    where
        K: Ord,
    {
        unsafe {
            let mut update: Vec<Link<K, V>> = vec![ptr::null_mut(); self.head_capacity()];
            let mut current = self.head;

            for i in (0..self.current_level).rev() {
                while !(&(*current).forward)[i].is_null()
                    && (*(&(*current).forward)[i]).key.as_ref().unwrap() < key
                {
                    current = (&(*current).forward)[i];
                }
                update[i] = current;
            }

            let target = (&(*current).forward)[0];
            if target.is_null() || (*target).key.as_ref().unwrap() != key {
                return None;
            }

            for i in 0..self.current_level {
                if (&(*update[i]).forward)[i] != target {
                    break;
                }
                (&mut (*update[i]).forward)[i] = (&(*target).forward)[i];
            }

            let mut target = Box::from_raw(target);
            let value = target.value.take();
            drop(target);

            self.length -= 1;

            while self.current_level > 1
                && (&(*self.head).forward)[self.current_level - 1].is_null()
            {
                self.current_level -= 1;
            }

            value
        }
    }

    pub fn copy(&self) -> Self
    where
        K: Clone,
        V: Clone,
    {
        unsafe {
            let new_head = Box::into_raw(Box::new(Node {
                key: None,
                value: None,
                forward: vec![ptr::null_mut(); self.head_capacity()],
                level: self.head_capacity(),
            }));

            let new_list = Self {
                head: new_head,
                length: self.length,
                current_level: self.current_level,
                insert_counter: self.insert_counter,
                probability: self.probability,
                rng_state: self.rng_state,
                level_mode: self.level_mode,
                max_level_mode: self.max_level_mode,
            };

            let mut map: HashMap<Link<K, V>, Link<K, V>> = HashMap::new();
            map.insert(self.head, new_head);

            let mut prev_new = new_head;
            let mut old = (&(*self.head).forward)[0];
            while !old.is_null() {
                let new_node = Box::into_raw(Box::new(Node {
                    key: (*old).key.clone(),
                    value: (*old).value.clone(),
                    forward: vec![ptr::null_mut(); (*old).level],
                    level: (*old).level,
                }));
                (&mut (*prev_new).forward)[0] = new_node;
                map.insert(old, new_node);
                prev_new = new_node;
                old = (&(*old).forward)[0];
            }

            for level in 1..self.current_level {
                let mut old_cur = (&(*self.head).forward)[level];
                let mut new_prev = new_head;
                while !old_cur.is_null() {
                    let new_cur = map[&old_cur];
                    (&mut (*new_prev).forward)[level] = new_cur;
                    new_prev = new_cur;
                    old_cur = (&(*old_cur).forward)[level];
                }
            }

            new_list
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            let mut current = (&(*self.head).forward)[0];
            while !current.is_null() {
                let next = (&(*current).forward)[0];
                drop(Box::from_raw(current));
                current = next;
            }

            for slot in (&mut (*self.head).forward).iter_mut() {
                *slot = ptr::null_mut();
            }

            self.length = 0;
            self.current_level = 1;
            self.insert_counter = 1;
        }
    }
}

impl<K, V> SkipList<K, V> {
    fn pick_level(&mut self) -> usize {
        let cap = self.resolve_max_level();
        match self.level_mode {
            LevelMode::Random => self.random_level(cap),
            LevelMode::Deterministic => self.deterministic_level(cap),
        }
    }

    fn random_level(&mut self, cap: Option<usize>) -> usize {
        let max = cap.unwrap_or(usize::MAX);
        let mut level = 1;
        while self.rand_float() < self.probability && level < max {
            level += 1;
        }
        level
    }

    fn deterministic_level(&mut self, cap: Option<usize>) -> usize {
        let level = 1 + self.insert_counter.trailing_zeros() as usize;
        self.insert_counter += 1;
        match cap {
            Some(m) => level.min(m),
            None => level,
        }
    }

    fn rand_float(&mut self) -> f64 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 17;
        self.rng_state ^= self.rng_state << 5;
        (self.rng_state as f64) / (u32::MAX as f64)
    }

    fn resolve_max_level(&self) -> Option<usize> {
        match self.max_level_mode {
            MaxLevelMode::Constant(n) => Some(n.max(1)),
            MaxLevelMode::Unlimited => None,
            MaxLevelMode::Function(f) => Some((f)(self.probability, self.length).max(1)),
        }
    }
}

impl<K, V> Display for SkipList<K, V>
where
    K: Display,
    V: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unsafe {
            let mut columns: Vec<Link<K, V>> = Vec::new();
            let mut current = (&(*self.head).forward)[0];
            while !current.is_null() {
                columns.push(current);
                current = (&(*current).forward)[0];
            }

            let labels: Vec<String> = columns.iter().map(|&n| format!("{}", &*n)).collect();
            let widths: Vec<usize> = labels.iter().map(|s| s.len()).collect();
            let prefix_width = format!("{}:", self.current_level.saturating_sub(1)).len();

            for level in (0..self.current_level).rev() {
                let mut next = (&(*self.head).forward)[level];
                let mut cells: Vec<String> = Vec::with_capacity(columns.len());

                for col in 0..columns.len() {
                    if !next.is_null() && next == columns[col] {
                        cells.push(labels[col].clone());
                        next = (&(*next).forward)[level];
                    } else {
                        cells.push(" ".repeat(widths[col]));
                    }
                }

                writeln!(
                    f,
                    "{:width$} {}",
                    format!("{level}:"),
                    cells.join(" -> "),
                    width = prefix_width
                )?;
            }
        }
        writeln!(f, "{}", self.current_level);
        Ok(())
    }
}

impl<K, V> Drop for SkipList<K, V> {
    fn drop(&mut self) {
        unsafe {
            let mut current = self.head;

            while !current.is_null() {
                let next = (&(*current).forward)[0];
                drop(Box::from_raw(current));
                current = next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_list() -> SkipList<i32, String> {
        SkipList::new(LevelMode::Deterministic, MaxLevelMode::Constant(4), 0.5)
    }

    #[test]
    fn test_insert_positive() {
        let mut list = sample_list();
        assert_eq!(list.len(), 0);

        list.insert(10, "val1".to_string());
        list.insert(20, "val2".to_string());

        assert_eq!(list.len(), 2);
        assert_eq!(list.search(&10), Some(&"val1".to_string()));
        assert_eq!(list.search(&20), Some(&"val2".to_string()));
    }

    #[test]
    fn test_insert_duplicate_key_updates_value_not_len() {
        let mut list = sample_list();
        list.insert(10, "old".to_string());
        assert_eq!(list.len(), 1);

        list.insert(10, "new".to_string());
        assert_eq!(list.len(), 1);
        assert_eq!(list.search(&10), Some(&"new".to_string()));
    }

    #[test]
    fn test_search_positive() {
        let mut list = sample_list();
        list.insert(1, "one".to_string());
        list.insert(2, "two".to_string());

        assert_eq!(list.search(&1), Some(&"one".to_string()));
        assert_eq!(list.search(&2), Some(&"two".to_string()));
    }

    #[test]
    fn test_search_negative_missing_key() {
        let mut list = sample_list();
        list.insert(1, "one".to_string());

        assert_eq!(list.search(&999), None);
        assert_eq!(list.search(&0), None);
    }

    #[test]
    fn test_delete_positive() {
        let mut list = sample_list();
        list.insert(5, "five".to_string());
        list.insert(10, "ten".to_string());

        let deleted = list.delete(&5);
        assert_eq!(deleted, Some("five".to_string()));
        assert_eq!(list.len(), 1);
        assert_eq!(list.search(&5), None);
    }

    #[test]
    fn test_delete_negative_non_existent() {
        let mut list = sample_list();
        list.insert(5, "five".to_string());

        let deleted = list.delete(&100);
        assert_eq!(deleted, None);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_len_and_is_empty_positive() {
        let mut list = sample_list();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        list.insert(1, "val".to_string());
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_len_after_failed_delete() {
        let mut list = sample_list();
        list.insert(1, "val".to_string());

        list.delete(&99);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_copy_positive() {
        let mut list = sample_list();
        list.insert(1, "a".to_string());
        list.insert(2, "b".to_string());

        let copy = list.copy();
        assert_eq!(copy.len(), 2);
        assert_eq!(copy.search(&1), Some(&"a".to_string()));
        assert_eq!(copy.search(&2), Some(&"b".to_string()));
    }

    #[test]
    fn test_copy_independence_negative_mutation() {
        let mut list = sample_list();
        list.insert(1, "original".to_string());

        let mut copy = list.copy();

        copy.insert(1, "modified_in_copy".to_string());
        copy.delete(&1);

        assert_eq!(copy.search(&1), None);
        assert_eq!(list.search(&1), Some(&"original".to_string()));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_clear_positive() {
        let mut list = sample_list();
        list.insert(1, "a".to_string());
        list.insert(2, "b".to_string());
        list.clear();

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.search(&1), None);
    }

    #[test]
    fn test_clear_already_empty_does_not_break() {
        let mut list = sample_list();
        list.clear();

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());

        list.insert(42, "answer".to_string());
        assert_eq!(list.len(), 1);
        assert_eq!(list.search(&42), Some(&"answer".to_string()));
    }
}
