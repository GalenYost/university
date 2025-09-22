use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct List<T> {
    vec: Vec<T>,
}

impl<T> List<T>
where
    T: Clone + Display,
{
    pub fn new(vec: Option<Vec<T>>) -> Self {
        if let Some(vec) = vec {
            Self { vec }
        } else {
            Self { vec: Vec::new() }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.vec.iter()
    }

    pub fn iter_inf(&self) -> impl Iterator<Item = &T> {
        self.vec.iter().cycle()
    }

    pub fn get_el_n(&self, index: usize) -> Option<&T> {
        if index > self.vec.len() {
            println!("out of bounds");
            return None;
        }
        self.vec.get(index)
    }

    pub fn get_every_nth(&self, index: usize) -> Vec<T> {
        if index == 0 {
            println!("n must be > 0");
            return vec![];
        }

        self.vec
            .iter()
            .enumerate()
            .filter_map(|(i, val)| {
                if (i + 1) % index == 0 {
                    Some(val.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn clear(&mut self) -> &mut Self {
        *self = List::new(None);
        self
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn insert_first(&mut self, el: T) -> &mut Self {
        self.vec.extend(vec![el]);
        self
    }

    pub fn insert_last(&mut self, el: T) -> &mut Self {
        self.vec.push(el);
        self
    }

    pub fn insert_after_n(&mut self, el: T, n: usize) -> &mut Self {
        if n + 1 > self.vec.len() {
            println!("out of bounds");
            return self;
        }
        self.vec.insert(n + 1, el);
        self
    }

    pub fn move_by(&mut self, index: usize, offset: isize) {
        let len = self.vec.len();
        if index >= len {
            println!("out of bounds");
            return;
        }

        let item = self.vec.remove(index);
        let mut new_index = (index as isize + offset).rem_euclid(len as isize);

        if new_index as usize >= self.vec.len() {
            new_index = self.vec.len() as isize;
        }

        self.vec.insert(new_index as usize, item);
    }

    pub fn remove_nth(&mut self, n: usize) -> &mut Self {
        if n > self.vec.len() {
            println!("out of bounds");
            return self;
        }
        self.vec.remove(n);
        self
    }

    pub fn remove_every_nth(&mut self, n: usize) -> &mut Self {
        if n == 0 {
            println!("n must be > 0");
            return self;
        }

        let mut i = self.vec.len();
        while i >= n {
            i -= n;
            self.vec.remove(i);
        }

        self
    }
}
