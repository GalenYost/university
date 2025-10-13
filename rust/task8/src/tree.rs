use std::{
    cell::RefCell,
    fmt::{self, Display, Formatter},
    rc::Rc,
    str::FromStr,
};

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
        let node = Node {
            val,
            left: None,
            right: None,
        };
        self.head = Some(Rc::new(RefCell::new(node)));
        self.path.push(self.head.clone().unwrap());
        self
    }

    pub fn push_under_ptr(&mut self, val: T, dir: PtrDirection) -> &mut Self {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            left: None,
            right: None,
        }));
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
