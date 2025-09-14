use crate::iter::{Iter, IterMut};
use crate::student::{SearchCriteria, Student};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct List {
    val: Student,
    next: Option<Box<List>>,
}

impl List {
    pub fn new(val: Student) -> Self {
        Self { val, next: None }
    }

    fn iter(&self) -> Iter<'_> {
        Iter { next: Some(self) }
    }

    fn iter_mut(&mut self) -> IterMut {
        IterMut { next: Some(self) }
    }

    pub fn current(&self) -> &Student {
        &self.val
    }

    pub fn next_ref(&self) -> Option<&List> {
        self.next.as_deref()
    }

    pub fn next_ref_mut(&mut self) -> Option<&mut List> {
        self.next.as_deref_mut()
    }

    pub fn get_element_n(&self, n: usize) -> Option<&List> {
        self.iter().nth(n)
    }

    pub fn clear(&mut self) -> () {
        self.next = None;
    }

    pub fn take_element_n(&mut self, n: usize) -> Option<Box<List>> {
        if n == 0 {
            let mut old_head = self.next.take();
            std::mem::swap(self, old_head.as_mut()?);
            return old_head;
        }

        let mut current: &mut List = self;
        for _ in 0..(n - 1) {
            current = current.next.as_mut()?;
        }

        current.next.take().map(|mut node| {
            current.next = node.next.take();
            node
        })
    }

    pub fn insert_first(&mut self, value: Student) -> &mut Self {
        let current_node = self.clone();

        let node = List {
            val: value,
            next: Some(Box::new(current_node)),
        };

        *self = node;
        self
    }

    pub fn insert_last(&mut self, value: Student) -> () {
        let mut current = self;
        while let Some(ref mut next) = current.next {
            current = next;
        }
        current.next = Some(Box::new(List::new(value)));
    }

    pub fn insert_value_after_n(&mut self, value: Student, n: usize) -> () {
        let mut current = self;
        let mut idx = 0;

        while idx < n {
            if current.next.is_some() {
                current = current.next.as_mut().unwrap();
                idx += 1;
            } else {
                current.next = Some(Box::new(List::new(value)));
                return;
            }
        }

        let mut new_list = List::new(value);
        new_list.next = current.next.take();
        current.next = Some(Box::new(new_list));
    }

    pub fn insert_element_after_n(&mut self, mut element: Box<List>, n: usize) -> () {
        let mut current = self;
        let mut idx = 0;

        while idx < n {
            if current.next.is_some() {
                current = current.next.as_mut().unwrap();
                idx += 1;
            } else {
                current.next = Some(element);
                return;
            }
        }

        element.next = current.next.take();
        current.next = Some(element);
    }

    pub fn move_for_n(&mut self, el_idx: usize, n: usize) -> () {
        if n == 0 {
            return;
        }

        if el_idx == 0 {
            let mut node_to_move = Box::new(List {
                val: self.val.clone(),
                next: self.next.take(),
            });

            let mut insert = &mut node_to_move.next;
            for _ in 0..(n - 1) {
                if let Some(next) = insert {
                    insert = &mut next.next;
                } else {
                    break;
                }
            }

            let tail = insert.take();
            *insert = Some(Box::new(List {
                val: self.val.clone(),
                next: tail,
            }));
            self.val = node_to_move.val;
            self.next = node_to_move.next;
            return;
        }

        let mut prev: &mut List = self;
        for _ in 0..(el_idx - 1) {
            if let Some(ref mut next) = prev.next {
                prev = next;
            } else {
                return;
            }
        }

        let mut node_to_move = match prev.next.take() {
            Some(node) => node,
            None => return,
        };
        prev.next = node_to_move.next.take();

        let mut insert = self;
        for _ in 0..(el_idx + n - 1) {
            if let Some(ref mut next) = insert.next {
                insert = next;
            } else {
                break;
            }
        }

        node_to_move.next = insert.next.take();
        insert.next = Some(node_to_move);
    }

    pub fn remove_nth(&mut self, n: usize) -> () {
        if let Some(mut el) = self.take_element_n(n) {
            if let Some(next) = el.next.take() {
                self.insert_element_after_n(next, n);
            }
            return;
        }
    }

    pub fn remove_every_n(&mut self, n: usize) -> () {
        if n == 0 {
            return;
        }
        let size = self.iter().count();

        for i in (0..size).rev() {
            if (i + 1) % n == 0 {
                self.remove_nth(i);
            }
        }
    }

    pub fn sort(&mut self, method: SearchCriteria, ascending: bool) -> () {
        let mut nodes: Vec<Box<List>> = Vec::new();
        let mut current = Some(Box::new(self.clone()));

        while let Some(mut node) = current {
            current = node.next.take();
            nodes.push(node);
        }

        match method {
            SearchCriteria::Name(_) => {
                match ascending {
                    true => {
                        nodes.sort_by(|a, b| a.current().get_info().0.cmp(b.current().get_info().0))
                    }
                    false => {
                        nodes.sort_by(|a, b| b.current().get_info().0.cmp(a.current().get_info().0))
                    }
                };
            }
            SearchCriteria::Surname(_) => {
                match ascending {
                    true => {
                        nodes.sort_by(|a, b| a.current().get_info().1.cmp(b.current().get_info().1))
                    }
                    false => {
                        nodes.sort_by(|a, b| b.current().get_info().1.cmp(a.current().get_info().1))
                    }
                };
            }
            SearchCriteria::Address(_) => {
                match ascending {
                    true => {
                        nodes.sort_by(|a, b| a.current().get_info().2.cmp(b.current().get_info().2))
                    }
                    false => {
                        nodes.sort_by(|a, b| b.current().get_info().2.cmp(a.current().get_info().2))
                    }
                };
            }
            SearchCriteria::Rating(_) => {
                match ascending {
                    true => {
                        nodes.sort_by(|a, b| a.current().get_info().4.cmp(b.current().get_info().4))
                    }
                    false => {
                        nodes.sort_by(|a, b| b.current().get_info().4.cmp(a.current().get_info().4))
                    }
                };
            }
            _ => unreachable!(),
        };

        if let Some(mut first) = nodes.clone().into_iter().next() {
            let mut current = &mut first;
            for node in nodes.into_iter().skip(1) {
                current.next = Some(node);
                current = current.next.as_mut().unwrap();
            }
            *self = *first;
        }
    }

    pub fn append(&mut self, other: List) -> () {
        let mut current = self;
        while let Some(ref mut next) = current.next {
            current = next;
        }
        current.next = Some(Box::new(other));
    }

    pub fn intersection(list1: &List, list2: &List) -> Option<List> {
        let values: Vec<Student> = list1
            .iter()
            .filter_map(|node1| {
                let student1 = &node1.val;
                if list2.iter().any(|node2| &node2.val == student1) {
                    Some(student1.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut iter = values.into_iter().rev();
        iter.next().map(|first| {
            let mut result = List::new(first);
            for val in iter {
                result.insert_first(val);
            }
            result
        })
    }
}
