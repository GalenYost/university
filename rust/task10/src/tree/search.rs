use super::{BinaryTree, Node};
use std::cell::RefCell;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

pub trait Traversal<T> {
    fn preorder<F: FnMut(&T)>(&self, visit: F);
    fn inorder<F: FnMut(&T)>(&self, visit: F);
    fn postorder<F: FnMut(&T)>(&self, visit: F);
}

impl<T> Traversal<T> for BinaryTree<T>
where
    T: Display + Clone,
{
    fn preorder<F: FnMut(&T)>(&self, mut visit: F) {
        if let Some(head) = &self.head {
            fn helper<T, F: FnMut(&T)>(node: &Option<Rc<RefCell<Node<T>>>>, visit: &mut F) {
                if let Some(n) = node {
                    let n = n.borrow();
                    visit(&n.val);
                    helper(&n.left, visit);
                    helper(&n.right, visit);
                }
            }
            helper(&Some(head.clone()), &mut visit);
        }
    }

    fn inorder<F: FnMut(&T)>(&self, mut visit: F) {
        if let Some(head) = &self.head {
            fn helper<T, F: FnMut(&T)>(node: &Option<Rc<RefCell<Node<T>>>>, visit: &mut F) {
                if let Some(n) = node {
                    let n = n.borrow();
                    helper(&n.left, visit);
                    visit(&n.val);
                    helper(&n.right, visit);
                }
            }
            helper(&Some(head.clone()), &mut visit);
        }
    }

    fn postorder<F: FnMut(&T)>(&self, mut visit: F) {
        if let Some(head) = &self.head {
            fn helper<T, F: FnMut(&T)>(node: &Option<Rc<RefCell<Node<T>>>>, visit: &mut F) {
                if let Some(n) = node {
                    let n = n.borrow();
                    helper(&n.left, visit);
                    helper(&n.right, visit);
                    visit(&n.val);
                }
            }
            helper(&Some(head.clone()), &mut visit);
        }
    }
}

impl<T> Display for BinaryTree<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn display_indented<T: Display>(
            f: &mut Formatter<'_>,
            node: &Option<Rc<RefCell<Node<T>>>>,
            current: Option<&Rc<RefCell<Node<T>>>>,
            depth: usize,
        ) -> fmt::Result {
            if let Some(n_rc) = node {
                let n = n_rc.borrow();

                display_indented(f, &n.right, current, depth + 1)?;

                for _ in 0..depth {
                    write!(f, "\t")?;
                }

                if let Some(cur_rc) = current {
                    if Rc::ptr_eq(cur_rc, n_rc) {
                        writeln!(f, "[*{}*]", n.val)?;
                    } else {
                        writeln!(f, "{}", n.val)?;
                    }
                } else {
                    writeln!(f, "{}", n.val)?;
                }

                display_indented(f, &n.left, current, depth + 1)?;
            }
            Ok(())
        }

        if self.head.is_none() {
            writeln!(f, "Tree is empty")?;
        } else {
            writeln!(f, "Current tree structure:")?;
            let current = self.path.last();
            display_indented(f, &self.head, current, 0)?;
        }

        Ok(())
    }
}
