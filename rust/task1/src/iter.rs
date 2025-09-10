use crate::list::List;

pub struct Iter<'a> {
    pub next: Option<&'a List>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a List;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next_ref();
            node
        })
    }
}

pub struct IterMut {
    pub next: Option<*mut List>,
}

impl IterMut {
    pub fn new(list: &mut List) -> Self {
        IterMut {
            next: Some(list as *mut _),
        }
    }
}

impl Iterator for IterMut {
    type Item = *mut List;

    fn next(&mut self) -> Option<Self::Item> {
        let node_ptr = self.next?;
        unsafe {
            let node = &mut *node_ptr;
            self.next = node.next_ref_mut().map(|b| b as *mut List);
            Some(node)
        }
    }
}
