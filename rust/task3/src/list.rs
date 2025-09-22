use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T>
where
    T: Clone + Display,
{
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, el: T) -> &mut Self {
        self.0.push(el);
        self
    }

    pub fn pop(&mut self) -> &mut Self {
        self.0.pop();
        self
    }

    pub fn switch(mut self) -> Self {
        if self.len() > 1 {
            self.0.swap(0, self.0.len() - 1);
        }
        self
    }
}
