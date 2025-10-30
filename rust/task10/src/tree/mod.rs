pub mod balance;
pub mod search;

use balance::Balance;
use search::Traversal;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;

pub enum PtrDirection {
    HEAD,
    UP,
    LEFT,
    RIGHT,
}

impl FromStr for PtrDirection {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "head" => Ok(PtrDirection::HEAD),
            "up" => Ok(PtrDirection::UP),
            "left" => Ok(PtrDirection::LEFT),
            "right" => Ok(PtrDirection::RIGHT),
            _ => Err("Wrong direction".to_string()),
        }
    }
}

pub enum SearchOrder {
    PREORDER,
    INORDER,
    POSTORDER,
}

impl FromStr for SearchOrder {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "pre" => Ok(SearchOrder::PREORDER),
            "in" => Ok(SearchOrder::INORDER),
            "post" => Ok(SearchOrder::POSTORDER),
            _ => Err("Wrong direction".to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    val: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
    height: usize,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
            height: 0,
        }
    }

    pub fn update_height(&mut self) -> usize {
        let left_height = self
            .left
            .as_ref()
            .map(|n| n.borrow_mut().update_height())
            .unwrap_or(0);

        let right_height = self
            .right
            .as_ref()
            .map(|n| n.borrow_mut().update_height())
            .unwrap_or(0);

        let height = 1 + left_height.max(right_height);
        self.height = height;
        height
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn balance_factor(&self) -> i8 {
        let left_height = self.left.as_ref().map_or(0, |l| l.borrow().height() as i8);
        let right_height = self.right.as_ref().map_or(0, |r| r.borrow().height() as i8);
        left_height - right_height
    }
}

pub struct BinaryTree<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    path: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> BinaryTree<T>
where
    T: Display + Clone,
{
    pub fn new() -> Self {
        Self {
            head: None,
            path: Vec::new(),
        }
    }

    pub fn insert_head(&mut self, val: T) -> &mut Self {
        let node = Node::new(val);
        self.head = Some(Rc::new(RefCell::new(node)));
        self.path.push(self.head.clone().unwrap());
        self
    }

    pub fn push_under_ptr(&mut self, val: T, dir: PtrDirection) -> &mut Self {
        let new_node = Rc::new(RefCell::new(Node::new(val)));
        if let Some(current) = self.path.last() {
            let mut borrowed = current.borrow_mut();
            match dir {
                PtrDirection::LEFT => borrowed.left = Some(new_node.clone()),
                PtrDirection::RIGHT => borrowed.right = Some(new_node.clone()),
                _ => println!("Only LEFT and RIGHT are possible"),
            }
        }
        self
    }

    pub fn remove_under_ptr(&mut self, dir: PtrDirection) -> &mut Self {
        if let Some(current) = self.path.last() {
            let mut borrowed = current.borrow_mut();
            match dir {
                PtrDirection::LEFT => borrowed.left = None,
                PtrDirection::RIGHT => borrowed.right = None,
                _ => println!("Only LEFT and RIGHT are possible"),
            }
        }
        self
    }

    pub fn move_ptr(&mut self, dir: PtrDirection) -> &mut Self {
        match dir {
            PtrDirection::HEAD => {
                if let Some(head) = &self.head {
                    self.path.clear();
                    self.path.push(head.clone());
                } else {
                    println!("No head");
                    return self;
                }
            }
            PtrDirection::UP => {
                if self.path.len() < 2 {
                    println!("Cant go beyond head");
                    return self;
                }
                self.path.pop();
                if self.path.is_empty() {
                    panic!("head has been removed")
                }
            }
            PtrDirection::LEFT => {
                let next = {
                    if let Some(last) = self.path.last() {
                        let node_ref = last.borrow();
                        node_ref.left.clone()
                    } else {
                        println!("Left is empty");
                        return self;
                    }
                };
                if let Some(left) = next {
                    self.path.push(left);
                }
            }
            PtrDirection::RIGHT => {
                let next = {
                    if let Some(last) = self.path.last() {
                        let node_ref = last.borrow();
                        node_ref.right.clone()
                    } else {
                        println!("Right is empty");
                        return self;
                    }
                };
                if let Some(right) = next {
                    self.path.push(right);
                }
            }
        }
        self
    }

    pub fn rebalance(&mut self) -> () {
        self.head = Some(Balance::<T>::rebalance(self.head.take().unwrap()));
    }

    pub fn search_for(&self, value: T, order: SearchOrder) -> Vec<T>
    where
        T: PartialEq,
    {
        let mut path: Vec<T> = Vec::new();
        let mut found = false;

        match order {
            SearchOrder::PREORDER => self.preorder(|val| {
                if !found {
                    path.push(val.clone());
                }
                if *val == value {
                    found = true
                }
            }),
            SearchOrder::INORDER => self.inorder(|val| {
                if !found {
                    path.push(val.clone());
                }
                if *val == value {
                    found = true
                }
            }),
            SearchOrder::POSTORDER => self.postorder(|val| {
                if !found {
                    path.push(val.clone());
                }
                if *val == value {
                    found = true
                }
            }),
        };

        path
    }
}

#[cfg(test)]
#[test]
fn insert() {
    let mut bt = BinaryTree::<usize>::new();
    assert_eq!(bt.head, None);

    bt.insert_head(100);
    assert_eq!(bt.head.clone().unwrap().borrow().val, 100);

    bt.push_under_ptr(110, PtrDirection::LEFT)
        .push_under_ptr(120, PtrDirection::RIGHT);
    assert_eq!(
        bt.head
            .clone()
            .unwrap()
            .borrow()
            .left
            .clone()
            .unwrap()
            .borrow()
            .val,
        110
    );
    assert_eq!(
        bt.head
            .clone()
            .unwrap()
            .borrow()
            .right
            .clone()
            .unwrap()
            .borrow()
            .val,
        120
    );
}

#[test]
fn node_height() {
    let rc = |n| Some(Rc::new(RefCell::new(Node::new(n))));

    let mut nd1 = Node::new(0);
    nd1.left = rc(1);
    nd1.right = rc(1);

    nd1.left.as_ref().unwrap().borrow_mut().left = rc(2);
    nd1.left.as_ref().unwrap().borrow_mut().right = rc(2);

    nd1.left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = rc(3);
    nd1.left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = rc(3);

    assert_eq!(nd1.update_height(), 4);
}
