use super::BinaryTree;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Balance<T> {
    fn rotate_left(&mut self) -> ();
    fn rotate_right(&mut self) -> ();
    fn rebalance(&mut self) -> ();
}

impl<T> Balance<T> for BinaryTree<T> {
    fn rebalance(&mut self) -> () {}
    fn rotate_left(&mut self) -> () {}
    fn rotate_right(&mut self) -> () {}
}
