use std::collections::BinaryHeap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Heap<T>(BinaryHeap<T>);

impl<T> Display for Heap<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            write!(f, "EMPTY")?;
            return Ok(());
        }

        writeln!(f, "")?;
        self.0.iter().rev().enumerate().for_each(|(i, el)| {
            write!(f, "{} - {}", i + 1, el)
                .map_err(|e| println!("{}", e))
                .ok();

            if i != self.0.len() - 1 {
                write!(f, "\n").map_err(|e| println!("{}", e)).ok();
            }
        });
        Ok(())
    }
}

impl<T> Heap<T>
where
    T: Clone + Display + Eq + PartialEq + Ord,
{
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, el: T) -> () {
        self.0.push(el);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn swap(&mut self) -> () {
        let cloned = self.0.clone();
        let mut v = cloned.into_sorted_vec();
        let last_idx = v.clone().len() - 1;
        v.swap(0, last_idx);
        self.0 = BinaryHeap::from(v);
    }

    pub fn reverse(&mut self) -> () {
        let cloned = self.0.clone();
        let mut v = cloned.into_sorted_vec();
        v.reverse();
        self.0 = BinaryHeap::from(v);
    }

    pub fn contains(&self, el: &T) -> bool
    where
        T: PartialEq,
    {
        self.0.iter().any(|x| x == el)
    }

    pub fn clear(&mut self) -> () {
        self.0.clear();
    }
}
