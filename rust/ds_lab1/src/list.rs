use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::{self, Display, Formatter};

pub type NodeRef<T> = Option<Rc<RefCell<Node<T>>>>;
pub type WeakNodeRef<T> = Option<Weak<RefCell<Node<T>>>>;

pub struct Node<T> {
    pub data: T,
    pub next: NodeRef<T>,
    pub prev: WeakNodeRef<T>,
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

pub struct DoublyLinkedList<T> {
    pub head: NodeRef<T>,
    pub tail: NodeRef<T>,
    pub len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn with_element(data: T) -> Self {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }));

        Self {
            head: Some(Rc::clone(&node)),
            tail: Some(node),
            len: 1,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.clone(),
            prev: None,
        }));

        match self.head.take() {
            Some(head) => {
                head.borrow_mut().prev = Some(Rc::downgrade(&node));
                self.head = Some(node);
            }
            None => {
                self.tail = Some(Rc::clone(&node));
                self.head = Some(node);
            }
        }

        self.len += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }));

        match self.tail.take() {
            Some(tail) => {
                node.borrow_mut().prev = Some(Rc::downgrade(&tail));
                tail.borrow_mut().next = Some(Rc::clone(&node));
                self.tail = Some(node);
            }
            None => {
                self.tail = Some(Rc::clone(&node));
                self.head = Some(node);
            }
        }

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.len -= 1;

            let next_node = old_head.borrow_mut().next.take();

            match next_node {
                Some(ref new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(Rc::clone(new_head));
                }
                None => {
                    self.tail = None;
                }
            }

            Rc::try_unwrap(old_head)
                .ok()
                .expect("Multiple references exist")
                .into_inner()
                .data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            self.len -= 1;

            let prev_node = old_tail
                .borrow_mut()
                .prev
                .take()
                .and_then(|weak| weak.upgrade());

            match prev_node {
                Some(ref new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(Rc::clone(new_tail));
                }
                None => {
                    self.head = None;
                }
            }

            Rc::try_unwrap(old_tail)
                .ok()
                .expect("Multiple references exist")
                .into_inner()
                .data
        })
    }

    pub fn swap_first_last(&mut self) {
        if self.len < 2 {
            return;
        }

        if let (Some(head), Some(tail)) = (&self.head, &self.tail) {
            std::mem::swap(
                &mut head.borrow_mut().data,
                &mut tail.borrow_mut().data,
            );
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn reverse(&mut self) {
        if self.len < 2 {
            return;
        }

        let mut current = self.head.clone();
        let mut prev_rc: Option<Rc<RefCell<Node<T>>>> = None;

        while let Some(node) = current {
            let next_node = node.borrow_mut().next.take();

            node.borrow_mut().next = prev_rc.clone();

            if let Some(ref prev) = prev_rc {
                prev.borrow_mut().prev = Some(Rc::downgrade(&node));
            }

            prev_rc = Some(node);
            current = next_node;
        }

        if let Some(ref new_head) = prev_rc {
            new_head.borrow_mut().prev = None;
        }

        std::mem::swap(&mut self.head, &mut self.tail);
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.len = 0;
    }
}

impl<T: Clone> DoublyLinkedList<T> {
    pub fn front(&self) -> Option<T> {
        self.head.as_ref().map(|node| node.borrow().data.clone())
    }

    pub fn back(&self) -> Option<T> {
        self.tail.as_ref().map(|node| node.borrow().data.clone())
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head.clone(),
            tail: self.tail.clone(),
        }
    }
}

impl<T> DoublyLinkedList<T> 
where
    T: Clone + PartialEq,
{
    pub fn contains(&self, target: &T) -> bool {
        self.iter().any(|el| &el == target)
    }
}

pub struct Iter<T> {
    head: NodeRef<T>,
    tail: NodeRef<T>,
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().map(|node| {
            let borrowed = node.borrow();
            self.head = borrowed.next.clone();
            borrowed.data.clone()
        })
    }
}

impl<T: Clone> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tail.take().map(|node| {
            let borrowed = node.borrow();
            self.tail = borrowed.prev.as_ref().and_then(|weak| weak.upgrade());
            borrowed.data.clone()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_empty() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);
    }

    #[test]
    fn test_with_element() {
        let list = DoublyLinkedList::with_element(42);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);
        assert_eq!(list.front(), Some(42));
        assert_eq!(list.back(), Some(42));
    }

    #[test]
    fn test_push_pop_front() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_pop_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(10);
        list.push_back(20);

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_back(), Some(20));
        assert_eq!(list.pop_back(), Some(10));
        assert_eq!(list.pop_back(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_interleaved_push_pop() {
        let mut list = DoublyLinkedList::new();
        list.push_front(2);
        list.push_back(3);
        list.push_front(1);

        assert_eq!(list.front(), Some(1));
        assert_eq!(list.back(), Some(3));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_swap_first_last() {
        let mut list = DoublyLinkedList::new();

        list.swap_first_last();

        list.push_back(1);
        list.swap_first_last();
        assert_eq!(list.front(), Some(1));

        list.push_back(2);
        list.push_back(3);
        list.swap_first_last();

        assert_eq!(list.front(), Some(3));
        assert_eq!(list.back(), Some(1));
        assert_eq!(list.iter().collect::<Vec<_>>(), vec![3, 2, 1]);
    }

    #[test]
    fn test_reverse() {
        let mut list = DoublyLinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        list.reverse();
        assert_eq!(list.iter().collect::<Vec<_>>(), vec![30, 20, 10]);
        assert_eq!(list.front(), Some(30));
        assert_eq!(list.back(), Some(10));

        list.reverse();
        assert_eq!(list.iter().collect::<Vec<_>>(), vec![10, 20, 30]);
    }

    #[test]
    fn test_contains() {
        let mut list = DoublyLinkedList::new();
        assert!(!list.contains(&10));

        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert!(list.contains(&10));
        assert!(list.contains(&20));
        assert!(list.contains(&30));
        assert!(!list.contains(&99));
    }

    #[test]
    fn test_clear() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_double_ended_iterator() {
        let mut list = DoublyLinkedList::new();
        list.push_back('a');
        list.push_back('b');
        list.push_back('c');

        let forward: Vec<_> = list.iter().collect();
        assert_eq!(forward, vec!['a', 'b', 'c']);

        let backward: Vec<_> = list.iter().rev().collect();
        assert_eq!(backward, vec!['c', 'b', 'a']);
    }
}
