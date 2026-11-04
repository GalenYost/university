use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Mul};
use std::ptr;

type Link<T> = *mut Node<T>;

#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    row: usize,
    col: usize,
    val: T,
    next_in_row: Link<T>,
    next_in_col: Link<T>,
}

impl<T> Node<T> {
    fn new(row: usize, col: usize, val: T) -> Self {
        Self {
            row,
            col,
            val,
            next_in_row: ptr::null_mut(),
            next_in_col: ptr::null_mut(),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<T> Default for Node<T>
where
    T: Default
{
    fn default() -> Self {
        Self {
            row: 0,
            col: 0,
            val: T::default(),
            next_in_row: ptr::null_mut(),
            next_in_col: ptr::null_mut(),
        }
    }
}

pub struct Matrix<T> {
    num_rows: usize,
    num_cols: usize,
    rows: Vec<Link<T>>,
    cols: Vec<Link<T>>,
}

impl<T> Default for Matrix<T> {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl<T> Matrix<T> {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        Self {
            num_rows,
            num_cols,
            rows: vec![ptr::null_mut(); num_rows],
            cols: vec![ptr::null_mut(); num_cols],
        }
    }

    pub fn rows(&self) -> usize {
        self.num_rows
    }

    pub fn cols(&self) -> usize {
        self.num_cols
    }

    pub fn non_zero_count(&self) -> usize {
        let mut count = 0;
        for &row in &self.rows {
            let mut curr = row;
            while !curr.is_null() {
                count += 1;
                curr = unsafe { (*curr).next_in_row };
            }
        }
        count
    }

    pub fn is_empty(&self) -> bool {
        self.num_rows == 0 || self.num_cols == 0
    }

    pub fn resize(&mut self, new_rows: usize, new_cols: usize) {
        if new_rows < self.num_rows || new_cols < self.num_cols {
            for r in 0..self.num_rows {
                let mut curr_link = &mut self.rows[r] as *mut Link<T>;
                while unsafe { !(*curr_link).is_null() } {
                    let node = unsafe { *curr_link };
                    let (nr, nc) = unsafe { ((*node).row, (*node).col) };

                    if nr >= new_rows || nc >= new_cols {
                        unsafe {
                            *curr_link = (*node).next_in_row;
                        }

                        let mut col_curr = &mut self.cols[nc] as *mut Link<T>;
                        while unsafe { !(*col_curr).is_null() && *col_curr != node } {
                            col_curr = unsafe { &mut (**col_curr).next_in_col as *mut Link<T> };
                        }
                        if unsafe { !(*col_curr).is_null() } {
                            unsafe {
                                *col_curr = (*node).next_in_col;
                            }
                        }

                        unsafe {
                            drop(Box::from_raw(node));
                        }
                    } else {
                        curr_link = unsafe { &mut (**curr_link).next_in_row as *mut Link<T> };
                    }
                }
            }
        }

        self.num_rows = new_rows;
        self.num_cols = new_cols;
        self.rows.resize(new_rows, ptr::null_mut());
        self.cols.resize(new_cols, ptr::null_mut());
    }

    pub fn expand_to_fit(&mut self, row: usize, col: usize) {
        let needed_rows = self.num_rows.max(row + 1);
        let needed_cols = self.num_cols.max(col + 1);
        if needed_rows > self.num_rows || needed_cols > self.num_cols {
            self.resize(needed_rows, needed_cols);
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.num_rows || col >= self.num_cols {
            return None;
        }

        let mut current = self.rows[row];
        while !current.is_null() {
            let curr_col = unsafe { (*current).col };
            if curr_col == col {
                return Some(unsafe { &(*current).val });
            }
            if curr_col > col {
                break;
            }
            current = unsafe { (*current).next_in_row };
        }
        None
    }

    pub fn set(&mut self, row: usize, col: usize, val: T)
    where
        T: PartialEq + Default,
    {
        if val == T::default() {
            if row >= self.num_rows || col >= self.num_cols {
                return;
            }

            let mut row_curr = &mut self.rows[row] as *mut Link<T>;
            unsafe {
                while !(*row_curr).is_null() && (**row_curr).col < col {
                    row_curr = &mut (**row_curr).next_in_row as *mut Link<T>;
                }

                if (*row_curr).is_null() || (**row_curr).col != col {
                    return;
                }

                let target = *row_curr;
                *row_curr = (*target).next_in_row;

                let mut col_curr = &mut self.cols[col] as *mut Link<T>;
                while !(*col_curr).is_null() && (**col_curr).row < row {
                    col_curr = &mut (**col_curr).next_in_col as *mut Link<T>;
                }

                if *col_curr == target {
                    *col_curr = (*target).next_in_col;
                }

                drop(Box::from_raw(target));
            }
        } else {
            self.expand_to_fit(row, col);

            let mut row_curr = &mut self.rows[row] as *mut Link<T>;
            unsafe {
                while !(*row_curr).is_null() && (**row_curr).col < col {
                    row_curr = &mut (**row_curr).next_in_row as *mut Link<T>;
                }

                if !(*row_curr).is_null() && (**row_curr).col == col {
                    (**row_curr).val = val;
                    return;
                }

                let mut col_curr = &mut self.cols[col] as *mut Link<T>;
                while !(*col_curr).is_null() && (**col_curr).row < row {
                    col_curr = &mut (**col_curr).next_in_col as *mut Link<T>;
                }

                let new_node = Box::into_raw(Box::new(Node {
                    next_in_row: *row_curr,
                    next_in_col: *col_curr,
                    ..Node::new(row, col, val)
                }));

                *row_curr = new_node;
                *col_curr = new_node;
            }
        }
    }

    pub fn mul_scalar(&self, scalar: T) -> Self
    where
        T: Copy + PartialEq + Default + Mul<Output = T>,
    {
        let mut result = Self::new(self.num_rows, self.num_cols);
        if scalar == T::default() {
            return result;
        }

        for &row_head in &self.rows {
            let mut current = row_head;
            while !current.is_null() {
                let node = unsafe { &*current };
                let new_val = node.val * scalar;
                if new_val != T::default() {
                    result.set(node.row, node.col, new_val);
                }
                current = node.next_in_row;
            }
        }

        result
    }

    pub fn clear(&mut self) {
        for row in self.rows.iter_mut() {
            let mut current = *row;
            while !current.is_null() {
                unsafe {
                    let next = (*current).next_in_row;
                    drop(Box::from_raw(current));
                    current = next;
                }
            }
            *row = ptr::null_mut();
        }
        for col in self.cols.iter_mut() {
            *col = ptr::null_mut();
        }
    }

    pub fn scale(&mut self, scalar: T)
    where
        T: Copy + PartialEq + Default + Mul<Output = T>,
    {
        if scalar == T::default() {
            self.clear();
            return;
        }

        for &row_head in &self.rows {
            let mut current = row_head;
            while !current.is_null() {
                unsafe {
                    (*current).val = (*current).val * scalar;
                    current = (*current).next_in_row;
                }
            }
        }
    }

    pub fn transpose(&self) -> Self
    where
        T: Copy + PartialEq + Default,
    {
        let mut result = Self::new(self.num_cols, self.num_rows);
        for &row_head in &self.rows {
            let mut current = row_head;
            while !current.is_null() {
                let node = unsafe { &*current };
                result.set(node.col, node.row, node.val);
                current = node.next_in_row;
            }
        }
        result
    }

    pub fn transpose_in_place(&mut self) {
        for &row_head in &self.rows {
            let mut current = row_head;
            while !current.is_null() {
                unsafe {
                    let next = (*current).next_in_row;
                    std::mem::swap(&mut (*current).row, &mut (*current).col);
                    std::mem::swap(&mut (*current).next_in_row, &mut (*current).next_in_col);
                    current = next;
                }
            }
        }
        std::mem::swap(&mut self.num_rows, &mut self.num_cols);
        std::mem::swap(&mut self.rows, &mut self.cols);
    }

    pub fn add(&self, other: &Self) -> Result<Self, String>
    where
        T: Copy + PartialEq + Default + Add<Output = T>,
    {
        if self.num_rows != other.num_rows || self.num_cols != other.num_cols {
            return Err(format!(
                "Cannot add matrices with different dimensions: ({}x{}) vs ({}x{})",
                self.num_rows, self.num_cols, other.num_rows, other.num_cols
            ));
        }

        let mut result = Self::new(self.num_rows, self.num_cols);

        for r in 0..self.num_rows {
            let mut curr_a = self.rows[r];
            let mut curr_b = other.rows[r];

            while !curr_a.is_null() || !curr_b.is_null() {
                let (col, sum_val) = match (curr_a.is_null(), curr_b.is_null()) {
                    (false, false) => {
                        let (node_a, node_b) = unsafe { (&*curr_a, &*curr_b) };
                        if node_a.col < node_b.col {
                            curr_a = node_a.next_in_row;
                            (node_a.col, node_a.val)
                        } else if node_b.col < node_a.col {
                            curr_b = node_b.next_in_row;
                            (node_b.col, node_b.val)
                        } else {
                            curr_a = node_a.next_in_row;
                            curr_b = node_b.next_in_row;
                            (node_a.col, node_a.val + node_b.val)
                        }
                    }
                    (false, true) => {
                        let node_a = unsafe { &*curr_a };
                        curr_a = node_a.next_in_row;
                        (node_a.col, node_a.val)
                    }
                    (true, false) => {
                        let node_b = unsafe { &*curr_b };
                        curr_b = node_b.next_in_row;
                        (node_b.col, node_b.val)
                    }
                    (true, true) => unreachable!(),
                };

                if sum_val != T::default() {
                    result.set(r, col, sum_val);
                }
            }
        }

        Ok(result)
    }

    pub fn add_assign(&mut self, other: &Self) -> Result<(), String>
    where
        T: Copy + PartialEq + Default + Add<Output = T>,
    {
        *self = self.add(other)?;
        Ok(())
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}x{}", self.num_rows, self.num_cols)?;

        if self.is_empty() {
            return writeln!(f, "Empty");
        }

        let row_prefix_width = format!("{}", self.num_rows.saturating_sub(1)).len();

        let mut col_width = 1;
        if self.num_cols > 0 {
            col_width = col_width.max(format!("{}", self.num_cols - 1).len());
        }
        for row in &self.rows {
            let mut current = *row;
            while !current.is_null() {
                let len = format!("{}", unsafe { &(*current).val }).len();
                col_width = col_width.max(len);
                current = unsafe { (*current).next_in_row };
            }
        }

        write!(f, "{:width$}", "", width = row_prefix_width + 3)?;
        for c in 0..self.num_cols {
            if c > 0 {
                write!(f, "  ")?;
            }
            write!(f, "{:>width$}", c, width = col_width)?;
        }
        writeln!(f)?;

        for r in 0..self.num_rows {
            write!(f, "{:>width$} [ ", r, width = row_prefix_width)?;

            let mut current = self.rows[r];
            for c in 0..self.num_cols {
                let val_str = if !current.is_null() && unsafe { (*current).col } == c {
                    let val = unsafe { &(*current).val };
                    current = unsafe { (*current).next_in_row };
                    format!("{}", val)
                } else {
                    "0".to_string()
                };

                if c > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:>width$}", val_str, width = col_width)?;
            }
            writeln!(f, " ]")?;
        }
        Ok(())
    }
}

impl<T> Drop for Matrix<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_and_orthogonal_links() {
        let mut mat = Matrix::<i32>::new(3, 4);
        mat.set(0, 2, 2);
        mat.set(0, 3, 1);
        mat.set(1, 0, 1);
        mat.set(1, 1, 2);
        mat.set(1, 2, 3);
        mat.set(2, 0, 2);
        mat.set(2, 2, 4);

        assert_eq!(mat.non_zero_count(), 7);
        assert_eq!(mat.get(0, 2), Some(&2));
        assert_eq!(mat.get(0, 3), Some(&1));
        assert_eq!(mat.get(1, 0), Some(&1));
        assert_eq!(mat.get(1, 1), Some(&2));
        assert_eq!(mat.get(1, 2), Some(&3));
        assert_eq!(mat.get(2, 0), Some(&2));
        assert_eq!(mat.get(2, 2), Some(&4));
        assert_eq!(mat.get(0, 0), None);
        assert_eq!(mat.get(2, 3), None);

        unsafe {
            let n1 = mat.cols[2];
            assert!(!n1.is_null());
            assert_eq!((*n1).row, 0);
            assert_eq!((*n1).val, 2);

            let n2 = (*n1).next_in_col;
            assert!(!n2.is_null());
            assert_eq!((*n2).row, 1);
            assert_eq!((*n2).val, 3);

            let n3 = (*n2).next_in_col;
            assert!(!n3.is_null());
            assert_eq!((*n3).row, 2);
            assert_eq!((*n3).val, 4);

            assert!((*n3).next_in_col.is_null());
        }

        mat.set(1, 1, 99);
        assert_eq!(mat.get(1, 1), Some(&99));
        assert_eq!(mat.non_zero_count(), 7);

        mat.set(0, 2, 0);
        assert_eq!(mat.get(0, 2), None);
        assert_eq!(mat.non_zero_count(), 6);

        unsafe {
            let n1 = mat.cols[2];
            assert!(!n1.is_null());
            assert_eq!((*n1).row, 1);
            assert_eq!((*n1).val, 3);
        }
    }

    #[test]
    fn test_auto_expansion_and_resize() {
        let mut mat = Matrix::<i32>::new(4, 4);
        assert_eq!(mat.rows(), 4);
        assert_eq!(mat.cols(), 4);

        mat.set(5, 6, 42);
        assert_eq!(mat.rows(), 6);
        assert_eq!(mat.cols(), 7);
        assert_eq!(mat.get(5, 6), Some(&42));
        assert_eq!(mat.get(0, 0), None);

        mat.resize(5, 5);
        assert_eq!(mat.rows(), 5);
        assert_eq!(mat.cols(), 5);
        assert_eq!(mat.get(5, 6), None);
        assert_eq!(mat.non_zero_count(), 0);
    }

    #[test]
    fn test_mul_scalar() {
        let mut mat = Matrix::<i32>::new(2, 3);
        mat.set(0, 1, 4);
        mat.set(1, 2, -5);

        let scaled = mat.mul_scalar(3);
        assert_eq!(scaled.rows(), 2);
        assert_eq!(scaled.cols(), 3);
        assert_eq!(scaled.get(0, 1), Some(&12));
        assert_eq!(scaled.get(1, 2), Some(&-15));
        assert_eq!(scaled.get(0, 0), None);
        assert_eq!(scaled.non_zero_count(), 2);

        let zeroed = mat.mul_scalar(0);
        assert_eq!(zeroed.rows(), 2);
        assert_eq!(zeroed.cols(), 3);
        assert_eq!(zeroed.non_zero_count(), 0);
        assert_eq!(zeroed.get(0, 1), None);

        mat.scale(-2);
        assert_eq!(mat.get(0, 1), Some(&-8));
        assert_eq!(mat.get(1, 2), Some(&10));
    }

    #[test]
    fn test_transpose() {
        let mut mat = Matrix::<i32>::new(3, 4);
        mat.set(0, 2, 2);
        mat.set(0, 3, 1);
        mat.set(1, 0, 1);
        mat.set(1, 1, 2);
        mat.set(1, 2, 3);
        mat.set(2, 0, 2);
        mat.set(2, 2, 4);

        let t = mat.transpose();
        assert_eq!(t.rows(), 4);
        assert_eq!(t.cols(), 3);
        assert_eq!(t.non_zero_count(), 7);

        assert_eq!(t.get(2, 0), Some(&2));
        assert_eq!(t.get(3, 0), Some(&1));
        assert_eq!(t.get(0, 1), Some(&1));
        assert_eq!(t.get(1, 1), Some(&2));
        assert_eq!(t.get(2, 1), Some(&3));
        assert_eq!(t.get(0, 2), Some(&2));
        assert_eq!(t.get(2, 2), Some(&4));
        assert_eq!(t.get(0, 0), None);

        mat.transpose_in_place();
        assert_eq!(mat.rows(), 4);
        assert_eq!(mat.cols(), 3);
        assert_eq!(mat.get(2, 0), Some(&2));
        assert_eq!(mat.get(0, 1), Some(&1));

        mat.transpose_in_place();
        assert_eq!(mat.rows(), 3);
        assert_eq!(mat.cols(), 4);
        assert_eq!(mat.get(0, 2), Some(&2));
        assert_eq!(mat.get(1, 0), Some(&1));
    }

    #[test]
    fn test_add() {
        let mut a = Matrix::<i32>::new(2, 3);
        a.set(0, 0, 5);
        a.set(0, 2, 3);
        a.set(1, 1, -7);

        let mut b = Matrix::<i32>::new(2, 3);
        b.set(0, 0, 10);
        b.set(0, 1, 4);
        b.set(1, 1, 7);

        let c = a.add(&b).unwrap();
        assert_eq!(c.rows(), 2);
        assert_eq!(c.cols(), 3);
        assert_eq!(c.get(0, 0), Some(&15));
        assert_eq!(c.get(0, 1), Some(&4)); 
        assert_eq!(c.get(0, 2), Some(&3)); 
        assert_eq!(c.get(1, 1), None);
        assert_eq!(c.non_zero_count(), 3);

        let d = Matrix::<i32>::new(3, 2);
        assert!(a.add(&d).is_err());

        a.add_assign(&b).unwrap();
        assert_eq!(a.get(0, 0), Some(&15));
        assert_eq!(a.get(1, 1), None);
    }
}
