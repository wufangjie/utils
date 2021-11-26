//! A AVL tree is a self-balancing binary search tree.
//!
//! version 0.1.2
//! https://github.com/wufangjie/learn/blob/main/src/utils/avl.rs
//!
//! This tree node use diff(balance factor) instead of regular height,
//! because, in most case, we do not interest in the height of a AVL tree,
//! and this change will reduce the memory usage (usize -> i8) and backtraces.
//!
//! Implemented pprint() for AVL tree Visualization.
//!
//! search_by(), remove_by() now return an Option,
//! since using Fn, we may not know the whole data.

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
pub struct AVL<T: Ord> {
    root: Option<Box<AVLNode<T>>>,
}

impl<T> AVL<T>
where
    T: Ord + fmt::Debug,
{
    pub fn new() -> Self {
        AVL { root: None }
    }

    pub fn search(&self, item: &T) -> bool {
        match self.search_by(|x| item.cmp(x)) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn search_by(&self, cmp: impl Fn(&T) -> Ordering) -> Option<&T> {
        let mut p = &self.root;
        while let Some(node) = p {
            match cmp(&node.data) {
                Ordering::Equal => return Some(&node.data),
                Ordering::Greater => p = &node.right,
                Ordering::Less => p = &node.left,
            }
        }
        None
    }

    pub fn insert(&mut self, item: T) {
        // insert_by is no need and dangerous
        let mut p = &mut self.root;
        let mut pre = p as *mut Option<Box<AVLNode<T>>>;
        let mut stack = vec![];
        while let Some(node) = p {
            if item < node.data {
                stack.push((pre, 1i8));
                p = &mut node.left;
            } else {
                stack.push((pre, -1i8));
                p = &mut node.right;
            }
            pre = p as *mut Option<Box<AVLNode<T>>>;
        }
        let mut node = Some(Box::new(AVLNode::new(item)));
        std::mem::swap(&mut node, p);

        // backtrace
        let mut diff = 0;
        while let Some((pre, flag)) = stack.pop() {
            let top = unsafe { &mut *pre };
            let node = &mut top.as_mut().unwrap();
            if node.diff == 0 {
                node.diff = flag;
            } else {
                if flag * node.diff > 0 {
                    let temp = node.diff;
                    Self::rebalance_i(top, temp, diff);
                } else {
                    node.diff = 0;
                }
                break;
            }
            diff = node.diff;
        }
    }

    pub fn height(&self) -> usize {
        let mut p = &self.root;
        let mut height = 0usize;
        while let Some(node) = p {
            height += 1;
            if node.diff >= 0 {
                p = &node.left;
            } else {
                p = &node.right;
            }
        }
        height
    }

    pub fn remove(&mut self, item: &T) {
        self.remove_by(|x| item.cmp(x));
    }

    pub fn remove_by(&mut self, cmp: impl Fn(&T) -> Ordering) -> Option<T> {
        // find node to remove
        let mut p = &mut self.root;
        let mut pre = p as *mut Option<Box<AVLNode<T>>>;
        let mut stack = vec![];
        let mut to_remove = std::ptr::null_mut();
        while let Some(node) = p {
            match cmp(&node.data) {
                Ordering::Equal => {
                    to_remove = pre;
                    match node.left {
                        None => {
                            stack.push((pre, -1i8));
                            p = &mut node.right;
                        }
                        Some(_) => {
                            stack.push((pre, 1i8));
                            p = &mut node.left;
                        }
                    }
                    pre = p as *mut Option<Box<AVLNode<T>>>;
                    break;
                }
                Ordering::Greater => {
                    stack.push((pre, -1i8));
                    p = &mut node.right;
                }
                Ordering::Less => {
                    stack.push((pre, 1i8));
                    p = &mut node.left;
                }
            }
            pre = p as *mut Option<Box<AVLNode<T>>>;
        }
        if to_remove.is_null() {
            return None;
        }

        // swap and remove
        while let Some(node) = p {
            stack.push((pre, -1i8));
            p = &mut node.right;
            pre = p as *mut Option<Box<AVLNode<T>>>;
        }
        let (p, _) = stack.pop().unwrap();
        let lr = unsafe { &mut *p };
        let mut ret = None::<Box<AVLNode<T>>>;
        if p == to_remove {
            std::mem::swap(lr, &mut ret);
        } else {
            let to_remove = unsafe { &mut *to_remove };
            std::mem::swap(
                &mut to_remove.as_mut().unwrap().data,
                &mut lr.as_mut().unwrap().data,
            );
            let lrl = lr.as_mut().unwrap().left.take();
            ret = std::mem::replace(lr, lrl);
        }

        // backtrace
        while let Some((pre, flag)) = stack.pop() {
            let top = unsafe { &mut *pre };
            let node = &mut top.as_mut().unwrap();
            if node.diff == 0 {
                node.diff = -flag;
                break;
            } else {
                if flag * node.diff < 0 {
                    let diff = node.diff;
                    if Self::rebalance_r(top, diff) {
                        break;
                    }
                } else {
                    node.diff = 0;
                }
            }
        }
        ret.map(|node| node.data)
    }

    pub fn iter_dfs(&self) -> IterDfs<'_, T> {
        let mut stack = vec![];
        if let Some(node) = &self.root {
            stack.push(&**node);
        }
        IterDfs { stack }
    }

    pub fn iter_bfs(&self) -> IterBfs<'_, T> {
        let mut queue = VecDeque::new();
        if let Some(node) = &self.root {
            queue.push_back(&**node);
        }
        IterBfs { queue }
    }

    pub fn pprint(&self) {
        if let Some(node) = &self.root {
            node.pprint_dfs("", " ");
        } else {
            println!(" ()");
        }
    }

    fn rebalance_i(top: &mut Option<Box<AVLNode<T>>>, diff: i8, diff_child: i8) {
        if diff == 1 {
            if diff_child >= 0 {
                Self::rotate_right(top);
                Self::update_diff(top, 1, 0);
                Self::update_diff(top, 0, 0);
            } else {
                Self::rotate_left(&mut top.as_mut().unwrap().left);
                Self::rotate_right(top);
                Self::update_diff_2r(top);
            }
        } else {
            if diff_child <= 0 {
                Self::rotate_left(top);
                Self::update_diff(top, -1, 0);
                Self::update_diff(top, 0, 0);
            } else {
                Self::rotate_right(&mut top.as_mut().unwrap().right);
                Self::rotate_left(top);
                Self::update_diff_2r(top);
            }
        }
    }

    #[allow(unused_must_use)]
    fn rotate_right(top: &mut Option<Box<AVLNode<T>>>) {
        let mut left = top.as_mut().unwrap().left.take();
        let lr = left.as_mut().unwrap().right.take();
        std::mem::replace(&mut top.as_mut().unwrap().left, lr);
        std::mem::swap(&mut left, top);
        std::mem::replace(&mut top.as_mut().unwrap().right, left);
    }

    #[allow(unused_must_use)]
    fn rotate_left(top: &mut Option<Box<AVLNode<T>>>) {
        let mut right = top.as_mut().unwrap().right.take();
        let rl = right.as_mut().unwrap().left.take();
        std::mem::replace(&mut top.as_mut().unwrap().right, rl);
        std::mem::swap(&mut right, top);
        std::mem::replace(&mut top.as_mut().unwrap().left, right);
    }

    fn rebalance_r(top: &mut Option<Box<AVLNode<T>>>, diff: i8) -> bool {
        if diff == 1 {
            let diff_child = top.as_mut().unwrap().left.as_mut().unwrap().diff;
            if diff_child == -1 {
                Self::rotate_left(&mut top.as_mut().unwrap().left);
                Self::rotate_right(top);
                Self::update_diff_2r(top)
            } else {
                Self::rotate_right(top);
                Self::update_diff_1r(top, diff, diff_child)
            }
        } else {
            let diff_child = top.as_mut().unwrap().right.as_mut().unwrap().diff;
            if diff_child == 1 {
                Self::rotate_right(&mut top.as_mut().unwrap().right);
                Self::rotate_left(top);
                Self::update_diff_2r(top)
            } else {
                Self::rotate_left(top);
                Self::update_diff_1r(top, diff, diff_child)
            }
        }
    }

    fn update_diff(top: &mut Option<Box<AVLNode<T>>>, which: i8, new: i8) {
        match which {
            -1 => top.as_mut().unwrap().left.as_mut().unwrap().diff = new,
            1 => top.as_mut().unwrap().right.as_mut().unwrap().diff = new,
            _ => top.as_mut().unwrap().diff = new,
        }
    }

    fn update_diff_2r(top: &mut Option<Box<AVLNode<T>>>) -> bool {
        // actually the top's diff is original grandchild's diff
        let (dl, dr) = match top.as_mut().unwrap().diff {
            -1 => (1, 0),
            1 => (0, -1),
            _ => (0, 0),
        };
        Self::update_diff(top, -1, dl);
        Self::update_diff(top, 1, dr);
        Self::update_diff(top, 0, 0);
        false
    }

    fn update_diff_1r(top: &mut Option<Box<AVLNode<T>>>, d1: i8, d2: i8) -> bool {
        if d2 == 0 {
            Self::update_diff(top, d1, d1);
            Self::update_diff(top, 0, -d1);
            true // no height change
        } else {
            Self::update_diff(top, d1, 0);
            Self::update_diff(top, 0, 0);
            false
        }
    }
}

impl<T> fmt::Display for AVL<T>
where
    T: Ord + fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        let mut is_first_time = true;
        for to_print in self.iter_bfs() {
            if is_first_time {
                is_first_time = false
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}", to_print)?;
        }
        write!(f, ")")
    }
}

#[derive(Debug)]
pub struct AVLNode<T: Ord> {
    data: T,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
    diff: i8, // left height - right height
}

impl<T> AVLNode<T>
where
    T: Ord,
{
    pub fn new(data: T) -> Self {
        AVLNode {
            data,
            left: None,
            right: None,
            diff: 0,
        }
    }
}

impl<T> AVLNode<T>
where
    T: Ord + fmt::Debug,
{
    fn pprint_dfs(&self, indent: &str, cur: &str) {
        if let Some(left) = &self.left {
            left.pprint_dfs(&Self::next_indent(indent, cur, "┗"), "┏");
        }
        println!("{}{}━{:?}", indent, cur, &self.data);
        if let Some(right) = &self.right {
            right.pprint_dfs(&Self::next_indent(indent, cur, "┏"), "┗");
        }
    }

    fn next_indent(indent: &str, pre: &str, not_cur: &str) -> String {
        String::from(indent) + if pre != not_cur { "  " } else { "┃ " }
    }
}

pub struct IterDfs<'a, T: Ord> {
    stack: Vec<&'a AVLNode<T>>,
}

impl<'a, T> Iterator for IterDfs<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // pre order
        match self.stack.pop() {
            None => None,
            Some(node) => {
                let ret = &node.data;
                if let Some(right) = &node.right {
                    self.stack.push(right);
                }
                if let Some(left) = &node.left {
                    self.stack.push(left);
                }
                Some(ret)
            }
        }
    }
}

pub struct IterBfs<'a, T: Ord> {
    queue: VecDeque<&'a AVLNode<T>>,
}

impl<'a, T> Iterator for IterBfs<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            None => None,
            Some(node) => {
                let ret = &node.data;
                if let Some(left) = &node.left {
                    self.queue.push_back(left);
                }
                if let Some(right) = &node.right {
                    self.queue.push_back(right);
                }
                Some(ret)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dbgt;

    impl<T> AVL<T>
    where
        T: Ord + fmt::Debug,
    {
        fn height2(p: &Option<Box<AVLNode<T>>>) -> isize {
            if let Some(node) = p {
                let hl = Self::height2(&node.left);
                let hr = Self::height2(&node.right);
                1 + if hl > hr { hl } else { hr }
            } else {
                0
            }
        }

        fn assert_diff(&self) {
            let mut queue = VecDeque::new();
            if let Some(root) = &self.root {
                queue.push_back(root);
            }
            while let Some(p) = queue.pop_front() {
                assert_eq!(
                    p.diff as isize,
                    Self::height2(&p.left) - Self::height2(&p.right)
                );
                if let Some(left) = &p.left {
                    queue.push_back(left);
                }
                if let Some(right) = &p.right {
                    queue.push_back(right);
                }
            }
        }

        fn inorder_dfs<'a>(node: &'a Option<Box<AVLNode<T>>>, res: &mut Vec<&'a T>) {
            if let Some(node) = node {
                Self::inorder_dfs(&node.left, res);
                res.push(&node.data);
                Self::inorder_dfs(&node.right, res);
            }
        }

        fn assert_valid_bst(&self) {
            let mut res = vec![];
            Self::inorder_dfs(&self.root, &mut res);
            let n = res.len();
            for i in 1..n {
                assert!(res[i - 1] <= res[i]);
            }
        }
    }

    // test case from:
    // https://stackoverflow.com/questions/3955680/how-to-check-if-my-avl-tree-implementation-is-correct

    #[test]
    fn test_avl() {
        let mut t1 = AVL::new();
        for i in [20, 4, 26, 3, 9, 15] {
            t1.insert(i);
        }
        assert_eq!(format!("{}", t1), "(9, 4, 20, 3, 15, 26)");
        // dbgt!(&t1);
        // println!("{}", t1);

        let mut t2 = AVL::new();
        for i in [20, 4, 26, 3, 9, 21, 30, 2, 7, 11, 15] {
            t2.insert(i);
        }
        assert_eq!(format!("{}", t2), "(9, 4, 20, 3, 7, 11, 26, 2, 15, 21, 30)");
        // dbgt!(&t2);
        // println!("{}", t2);

        let mut t3 = AVL::new();
        for i in [20, 4, 26, 3, 9, 8] {
            t3.insert(i);
        }
        assert_eq!(format!("{}", t3), "(9, 4, 20, 3, 8, 26)");
        // dbgt!(&t3);
        // println!("{}", t3);

        let mut t4 = AVL::new();
        for i in [20, 4, 26, 3, 9, 21, 30, 2, 7, 11, 8] {
            t4.insert(i);
        }
        assert_eq!(format!("{}", t4), "(9, 4, 20, 3, 7, 11, 26, 2, 8, 21, 30)");
        // dbgt!(&t4);
        // println!("{}", t4);

        assert_eq!(4, t4.height());
        assert!(t4.search(&8));
        assert!(!t4.search(&88));
        assert_eq!(Some(&8), t4.search_by(|x| 8.cmp(x)));
        assert_eq!(None, t4.search_by(|x| 88.cmp(x)));

        ////////////////////////////////////////////////////////////////////////
        // delete
        ////////////////////////////////////////////////////////////////////////
        let mut t5 = AVL::new();
        for i in [2, 1, 4, 3, 5] {
            t5.insert(i);
        }
        assert_eq!(Some(1), t5.remove_by(|x| 1.cmp(x)));
        assert_eq!(format!("{}", t5), "(4, 2, 5, 3)");
        // dbgt!(&t5);
        // println!("{}", t5);

        let mut t6 = AVL::new();
        for i in [6, 2, 9, 1, 4, 8, 66, 3, 5, 7, 65, 67, 68] {
            t6.insert(i);
        }
        t6.remove(&1);
        assert_eq!(
            format!("{}", t6),
            "(6, 4, 9, 2, 5, 8, 66, 3, 7, 65, 67, 68)"
        );
        // dbgt!(&t6);
        // println!("{}", t6);

        let mut t7 = AVL::new();
        for i in [5, 2, 8, 1, 3, 7, 65, 4, 6, 9, 66, 67] {
            t7.insert(i);
        }
        t7.remove(&1);
        assert_eq!(format!("{}", t7), "(8, 5, 65, 3, 7, 9, 66, 2, 4, 6, 67)");
        // dbgt!(&t7);
        // println!("{}", t7);

        ////////////////////////////////////////////////////////////////////////
        // iter
        ////////////////////////////////////////////////////////////////////////
        assert_eq!(
            t7.iter_dfs().collect::<Vec<&i32>>(),
            [8, 5, 3, 2, 4, 7, 6, 65, 9, 66, 67]
                .iter()
                .collect::<Vec<&i32>>()
        );
        assert_eq!(
            t7.iter_bfs().collect::<Vec<&i32>>(),
            [8, 5, 65, 3, 7, 9, 66, 2, 4, 6, 67]
                .iter()
                .collect::<Vec<&i32>>()
        );

        ////////////////////////////////////////////////////////////////////////
        // diff is correct
        ////////////////////////////////////////////////////////////////////////
        t1.assert_diff();
        t2.assert_diff();
        t3.assert_diff();
        t4.assert_diff();
        t5.assert_diff();
        t6.assert_diff();
        t7.assert_diff();

        ////////////////////////////////////////////////////////////////////////
        // is valid bst
        ////////////////////////////////////////////////////////////////////////
        // t1.assert_valid_bst();
        // t2.assert_valid_bst();
        // t3.assert_valid_bst();
        // t4.assert_valid_bst();
        // t5.assert_valid_bst();
        // t6.assert_valid_bst();
        // t7.assert_valid_bst();

        let mut t8 = AVL::new();
        for i in [
            8, 94, 4, 50, 17, 15, 37, 1, 25, 42, 39, 13, 83, 32, 89, 24, 6, 70, 90, 22, 10, 11, 68,
            72, 49, 99, 45, 19, 38, 28, 63, 16, 77, 46, 65, 33, 34, 60, 53, 54, 40, 84, 2, 56, 57,
            44, 59, 92, 95, 41, 98, 97, 80, 29, 87, 18, 26, 67, 79, 88, 30, 20, 35, 81, 78, 55, 12,
            43, 85, 82, 0, 62, 96, 61, 71, 23, 9, 74, 27, 91, 69, 76, 52, 47, 64, 86, 75, 5, 7, 36,
            21, 93, 66, 31, 50, 17, 58, 14, 3, 51, 48, 73, // 50, 17 insert twice
        ] {
            t8.insert(i);
        }
        t8.assert_valid_bst();
        for i in [8, 94, 4, 50, 17, 15, 37, 1, 25] {
            t8.remove(&i);
            t8.assert_valid_bst();
        }
        // t8.pprint();

        let mut t9 = AVL::new();
        for i in [
            (4, 5),
            (4, 2),
            (0, 8),
            (1, 1),
            (1, 3),
            (2, 7),
            (2, 4),
            (3, 6),
            (3, 9),
        ] {
            t9.insert(i);
        }
        t9.pprint();
        assert_eq!(Some((4, 2)), t9.remove_by(|x| 4.cmp(&x.0)));
        t9.pprint();
    }
}
