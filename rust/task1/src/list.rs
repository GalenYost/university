#[derive(Debug, Clone)]
pub struct List {
    val: usize,
    next: Option<Box<List>>,
}

impl List {
    pub fn new(initial_value: usize) -> Self {
        Self {
            val: initial_value,
            next: None,
        }
    }

    pub fn current(&self) -> usize {
        self.val
    }

    pub fn next(&self) -> Option<&List> {
        self.next.as_ref().map(|v| &**v)
    }

    pub fn get_element_n(&self, n: usize) -> Option<&List> {
        let mut current = self;
        let mut idx = 0;

        while idx < n {
            match current.next.as_ref() {
                Some(next) => {
                    current = next;
                    idx += 1;
                }
                None => return None,
            };
        }

        Some(current)
    }

    pub fn insert_first(self, value: usize) -> Self {
        Self {
            val: value,
            next: Some(Box::new(self)),
        }
    }

    pub fn insert_last(&mut self, value: usize) -> () {
        let mut current = self;
        while let Some(ref mut next) = current.next {
            current = next;
        }
        current.next = Some(Box::new(List::new(value)));
    }

    pub fn insert_after_n(&mut self, value: usize, n: usize) -> () {
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

    pub fn move_for_n(&mut self, el_idx: usize, n: usize) {
        if n == 0 {
            return;
        }

        if el_idx == 0 {
            let mut node_to_move = Box::new(List {
                val: self.val,
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
                val: self.val,
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
}
