use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Stack<T>(Vec<T>);

impl<T> Display for Stack<T>
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

impl<T> Stack<T>
where
    T: Clone + Display + Eq + PartialEq,
{
    pub fn new() -> Self {
        Self(Vec::new())
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
        if self.len() > 1 {
            let len = self.len();
            unsafe {
                let first: *mut _ = &mut self.0[len - 1];
                let last: *mut _ = &mut self.0[0];
                std::ptr::swap(first, last);
            }
        }
    }

    pub fn reverse(&mut self) -> () {
        self.0.reverse();
    }

    pub fn contains(&self, el: &T) -> bool {
        self.0.contains(el)
    }

    pub fn clear(&mut self) -> () {
        self.0.clear();
    }
}

#[test]
fn push() {
    let mut stack = Stack::<String>::new();
    stack.push("hello".into());
    stack.push("world".into());

    assert_eq!(stack.0[0], "hello".to_string());
    assert_eq!(stack.0[1], "world".to_string());
}

#[test]
fn pop() {
    let mut stack = Stack::<String>::new();
    stack.push("hello".into());
    stack.push("world".into());

    assert_eq!(stack.0[1], "world".to_string());

    stack.pop();

    assert_eq!(stack.0[0], "hello".to_string());
    assert_eq!(stack.0.get(1), None);
}

#[test]
fn swap() {
    let mut stack = Stack::<String>::new();
    stack.push("hello".into());
    stack.push("world".into());
    stack.push("!".into());

    stack.swap();

    assert_eq!(stack.0.first(), Some(&"!".to_string()));
    assert_eq!(stack.0.last(), Some(&"hello".to_string()));
}

#[test]
fn reverse() {
    let mut stack = Stack::<String>::new();
    stack.push("hello".into());
    stack.push(",".into());
    stack.push("world".into());
    stack.push("!".into());

    stack.reverse();

    assert_eq!(stack.0.first(), Some(&"!".to_string()));
    assert_eq!(stack.0.get(1), Some(&"world".to_string()));
    assert_eq!(stack.0.get(2), Some(&",".to_string()));
    assert_eq!(stack.0.last(), Some(&"hello".to_string()));
}

#[test]
fn contains() {
    let mut stack = Stack::<String>::new();
    stack.push("hello".into());
    stack.push(",".into());
    stack.push("world".into());
    stack.push("!".into());

    assert!(stack.contains(&"world".to_string()));
    assert!(!stack.contains(&"a".to_string()));
}

#[test]
fn clear() {
    let mut stack = Stack::<String>::new();
    stack.push("hello".into());
    stack.push(",".into());
    stack.push("world".into());
    stack.push("!".into());

    stack.clear();

    assert!(stack.0.is_empty());
}
