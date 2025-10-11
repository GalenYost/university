use std::{cell::RefCell, rc::Rc};

enum PtrDirection {
    HEAD,
    UP,
    LEFT,
    RIGHT,
}

pub struct Node<T> {
    val: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

pub struct BinaryTree<T> {
    head: Rc<RefCell<Node<T>>>,
    ptr: Rc<RefCell<Node<T>>>,
    path: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> BinaryTree<T> {
    pub fn new(val: T) -> Self {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            left: None,
            right: None,
        }));
        Self {
            head: new_node.clone(),
            ptr: new_node.clone(),
            path: Vec::new(),
        }
    }

    pub fn add(&mut self, val: T, left: bool) -> &mut Self {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            left: None,
            right: None,
        }));

        if left {
            self.ptr.borrow_mut().left = Some(new_node.clone());
        } else {
            self.ptr.borrow_mut().right = Some(new_node.clone());
        }

        self
    }

    pub fn remove(&mut self, val: T) -> &mut Self {
        self
    }
}
