use super::{BinaryTree, Node, PtrDirection};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Balance<T> {
    fn rotate_left(node: Rc<RefCell<Self>>) -> Rc<RefCell<Self>>;
    fn rotate_right(node: Rc<RefCell<Self>>) -> Rc<RefCell<Self>>;
    fn rebalance(node: Rc<RefCell<Self>>) -> Rc<RefCell<Self>>;
}

impl<T> Balance<T> for Node<T> {
    fn rebalance(node: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        node.borrow_mut().update_height();
        let balance = node.borrow().balance_factor();

        if balance > 1 {
            let left_balance = node
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .balance_factor();
            if left_balance >= 0 {
                return Self::rotate_right(node);
            } else {
                let left = node.borrow_mut().left.take().unwrap();
                node.borrow_mut().left = Some(Self::rotate_left(left));
                return Self::rotate_right(node);
            }
        } else if balance < -1 {
            let right_balance = node
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .balance_factor();
            if right_balance <= 0 {
                return Self::rotate_left(node);
            } else {
                let right = node.borrow_mut().right.take().unwrap();
                node.borrow_mut().right = Some(Self::rotate_right(right));
                return Self::rotate_left(node);
            }
        }
        node
    }

    fn rotate_left(z: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let mut z_borrow = z.borrow_mut();
        let y = z_borrow.right.take().unwrap();
        drop(z_borrow);

        let mut y_borrow = y.borrow_mut();
        let t2 = y_borrow.left.take();
        drop(y_borrow);

        z.borrow_mut().right = t2;

        y.borrow_mut().left = Some(z.clone());

        z.borrow_mut().update_height();
        y.borrow_mut().update_height();

        y
    }

    fn rotate_right(z: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let mut z_borrow = z.borrow_mut();
        let y = z_borrow.left.take().unwrap();
        drop(z_borrow);

        let mut y_borrow = y.borrow_mut();
        let t3 = y_borrow.right.take();
        drop(y_borrow);

        z.borrow_mut().left = t3;
        y.borrow_mut().right = Some(z.clone());

        z.borrow_mut().update_height();
        y.borrow_mut().update_height();

        y
    }
}

#[cfg(test)]
#[test]
fn rotations() {
    let mut bt = BinaryTree::<char>::new();

    bt.insert_head('a');

    bt.move_ptr(PtrDirection::LEFT);
    bt.push_under_ptr('b', PtrDirection::LEFT);
    bt.push_under_ptr('c', PtrDirection::RIGHT);

    bt.move_ptr(PtrDirection::RIGHT);
    bt.push_under_ptr('d', PtrDirection::LEFT);

    bt.move_ptr(PtrDirection::LEFT);
    bt.push_under_ptr('f', PtrDirection::LEFT);
    bt.push_under_ptr('g', PtrDirection::RIGHT);

    bt.move_ptr(PtrDirection::HEAD);

    bt.rebalance();

    assert_eq!(bt.head.as_ref().unwrap().borrow().val, 'd');
    assert_eq!(
        bt.head
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .unwrap()
            .borrow()
            .val,
        'a'
    );
    assert_eq!(
        bt.head
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .val,
        'c'
    );
}
