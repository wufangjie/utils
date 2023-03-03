//! A Avl tree is a self-balancing binary search tree.
//!
//! version 0.2.2
//! https://github.com/wufangjie/utils/blob/main/src/avl.rs
//!
//! This tree node use diff (balance factor) instead of regular height,
//! because, in most case, we do not interest in the height of an Avl tree,
//! and this change will reduce the memory usage (usize -> i8)
//!
//! Implemented pprint() for Avl tree Visualization.
//!
//! search_by(), remove_by() now return an Option,
//! since using Fn, we may not know the whole data (partial condition).
//!
//! Removing unsafe code (using recursive instead)
//! iter_dfs: preorder to inorder (so that we can make an ordered map)

use std::cmp::Ordering;
use std::fmt;
use std::mem;

#[derive(Debug)]
pub struct Avl<T: Ord> {
    root: Option<Box<AvlNode<T>>>,
}

impl<T: Ord> Default for Avl<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> Avl<T> {
    pub fn new() -> Self {
        Avl { root: None }
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

    pub fn iter_dfs(&self) -> IterDfs<'_, T> {
        let mut stack = Vec::<&AvlNode<T>>::new(); //vec![];
        let mut p = &self.root;
        if let Some(node) = p {
            stack.push(node);
            p = &node.left;
            while let Some(node) = p {
                stack.push(node);
                p = &node.left;
            }
        }
        IterDfs { stack }
    }
}

/// impl: search, insert and remove
impl<T: Ord> Avl<T> {
    pub fn search(&self, item: &T) -> bool {
        self.search_by(|x| item.cmp(x)).is_some()
        // match self.search_by(|x| item.cmp(x)) {
        //     Some(_) => true,
        //     None => false,
        // }
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

    /// return false (not insert) if exist one node.data == item
    pub fn insert(&mut self, item: T) -> bool {
        Self::insert_rec(&mut self.root, item).1
    }

    /// remove a node (node.data == item)
    pub fn remove(&mut self, item: &T) {
        self.remove_by(|x| item.cmp(x));
    }

    /// remove a node by a compare closure
    /// monotonically increasing required
    /// cmp's Greater means node.data is not big enough, then will go right branch
    pub fn remove_by(&mut self, cmp: impl Fn(&T) -> Ordering) -> Option<T> {
        Self::remove_by_rec(&mut self.root, cmp).1
    }

    /// rotate right without updating diff
    fn rotate_right(top: &mut Option<Box<AvlNode<T>>>) {
        let mut left = top.as_mut().unwrap().left.take();
        let lr = left.as_mut().unwrap().right.take();
        top.as_mut().unwrap().left = lr;
        mem::swap(&mut left, top);
        top.as_mut().unwrap().right = left;
    }

    /// rotate left without updating diff
    fn rotate_left(top: &mut Option<Box<AvlNode<T>>>) {
        let mut right = top.as_mut().unwrap().right.take();
        let rl = right.as_mut().unwrap().left.take();
        top.as_mut().unwrap().right = rl;
        mem::swap(&mut right, top);
        top.as_mut().unwrap().left = right;
    }

    /// diff only can be 1 or -1, actually the current real diff is 2 or -2
    /// return true means keeping the origin height (i.e. no need to backtrace)
    /// the return value is only for removing
    /// for inserting, we will always need balance once (since we insert one by one)
    fn rebalance(top: &mut Option<Box<AvlNode<T>>>, diff: i8) -> bool {
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

    /// update diff after twice rotating
    /// NOTE: final top's left and right children's diffs only depend on the original grandchild's diff
    /// after twice rotating, current top is actually the original grandchild
    /// finally the top node's diff always equal to 0 and no need backtrace any more
    fn update_diff_2r(top: &mut Option<Box<AvlNode<T>>>) -> bool {
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

    /// all possible cases: (1, 0), (1, 1), (-1, 0), (-1, -1)
    /// return true means keep the original (before inserting or removing) height, no need to backtrace
    fn update_diff_1r(top: &mut Option<Box<AvlNode<T>>>, d1: i8, d2: i8) -> bool {
        if d2 == 0 {
            Self::update_diff(top, d1, d1);
            Self::update_diff(top, 0, -d1);
            true
        } else {
            Self::update_diff(top, d1, 0);
            Self::update_diff(top, 0, 0);
            false
        }
    }

    /// reset one node's diff or its (left | right) child's diff
    #[inline]
    fn update_diff(top: &mut Option<Box<AvlNode<T>>>, which: i8, new: i8) {
        match which {
            -1 => top.as_mut().unwrap().left.as_mut().unwrap().diff = new,
            1 => top.as_mut().unwrap().right.as_mut().unwrap().diff = new,
            _ => top.as_mut().unwrap().diff = new,
        }
    }

    /// reset diff through its child (backtrace)
    /// which: {-1, 1} means backtrace from left or right child
    /// change: {-1, 0, 1} means the specific child's depth decreased, not change or increased
    fn backtrace(node: &mut Option<Box<AvlNode<T>>>, which: i8, change: i8) -> i8 {
        let diff = node.as_ref().unwrap().diff;
        match change {
            0 => 0,
            1 => match diff * which {
                0 => {
                    node.as_mut().unwrap().diff = -which;
                    1
                }
                1 => {
                    node.as_mut().unwrap().diff = 0;
                    0
                }
                -1 => {
                    Self::rebalance(node, diff);
                    0
                }
                _ => unreachable!(),
            },
            -1 => match diff * which {
                0 => {
                    node.as_mut().unwrap().diff = which;
                    0
                }
                -1 => {
                    node.as_mut().unwrap().diff = 0;
                    -1
                }
                1 => {
                    if Self::rebalance(node, diff) {
                        0
                    } else {
                        -1
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    /// i8: {0, 1} means changed 0 depth or increased 1 depth
    fn insert_rec(node: &mut Option<Box<AvlNode<T>>>, item: T) -> (i8, bool) {
        if node.is_none() {
            let mut leaf = Some(Box::new(AvlNode::new(item)));
            mem::swap(node, &mut leaf);
            return (1, true);
        }
        match item.cmp(&node.as_ref().unwrap().data) {
            Ordering::Equal => (0, false),
            Ordering::Greater => {
                let (mut delta, succeed) =
                    Self::insert_rec(&mut node.as_mut().unwrap().right, item);
                delta = Self::backtrace(node, 1, delta);
                (delta, succeed)
            }
            Ordering::Less => {
                let (mut delta, succeed) = Self::insert_rec(&mut node.as_mut().unwrap().left, item);
                delta = Self::backtrace(node, -1, delta);
                (delta, succeed)
            }
        }
    }

    /// use recursive to keep original &mut node, this can void using unsafe code
    /// but it seems can not convert to stack based code
    /// i8: {0, -1} means changed 0 depth or decreased 1 depth
    fn remove_by_rec(
        node: &mut Option<Box<AvlNode<T>>>,
        cmp: impl Fn(&T) -> Ordering,
    ) -> (i8, Option<T>) {
        if node.is_none() {
            return (0, None);
        }
        match cmp(&node.as_ref().unwrap().data) {
            Ordering::Equal => {
                let inner = node.as_mut().unwrap();
                if inner.left.is_none() {
                    if inner.right.is_none() {
                        (-1, Some(node.take().unwrap().data))
                    } else {
                        // this right must be a leaf node
                        let mut right = node.as_mut().unwrap().right.take();
                        mem::swap(node, &mut right);
                        (-1, Some(right.unwrap().data))
                    }
                } else {
                    let (mut delta, mut removed) = Self::remove_right_most_rec(&mut inner.left);
                    mem::swap(&mut inner.data, &mut removed.data);
                    delta = Self::backtrace(node, -1, delta);
                    (delta, Some(removed.data))
                }
            }
            Ordering::Greater => {
                let (mut delta, ret) = Self::remove_by_rec(&mut node.as_mut().unwrap().right, cmp);
                delta = Self::backtrace(node, 1, delta);
                (delta, ret)
            }
            Ordering::Less => {
                let (mut delta, ret) = Self::remove_by_rec(&mut node.as_mut().unwrap().left, cmp);
                delta = Self::backtrace(node, -1, delta);
                (delta, ret)
            }
        }
    }

    /// find right most node, replace it with its left children, then return
    /// it is commonly used for finding predecessor
    /// NOTE: before calling this recursive function, make sure `node` is not None
    /// return (child depth changed, removed)
    fn remove_right_most_rec(node: &mut Option<Box<AvlNode<T>>>) -> (i8, Box<AvlNode<T>>) {
        if node.as_mut().unwrap().right.is_some() {
            let (mut delta, ret) = Self::remove_right_most_rec(&mut node.as_mut().unwrap().right);
            delta = Self::backtrace(node, 1, delta);
            (delta, ret)
        } else {
            let mut left = node.as_mut().unwrap().left.take();
            mem::swap(node, &mut left);
            (-1, left.unwrap())
        }
    }
}

impl<T> Avl<T>
where
    T: Ord + fmt::Debug,
{
    pub fn pprint(&self) {
        if let Some(node) = &self.root {
            node.pprint_dfs("", " ");
        } else {
            println!(" ()");
        }
    }
}

#[derive(Debug)]
pub struct AvlNode<T: Ord> {
    data: T,
    left: Option<Box<AvlNode<T>>>,
    right: Option<Box<AvlNode<T>>>,
    diff: i8, // left height - right height
}

impl<T> AvlNode<T>
where
    T: Ord,
{
    pub fn new(data: T) -> Self {
        AvlNode {
            data,
            left: None,
            right: None,
            diff: 0,
        }
    }
}

impl<T> AvlNode<T>
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
    stack: Vec<&'a AvlNode<T>>,
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
                let mut p = &node.right;
                if let Some(node) = p {
                    self.stack.push(node);
                    p = &node.left;
                    while let Some(node) = p {
                        self.stack.push(node);
                        p = &node.left;
                    }
                }
                // let mut left = node.left;
                // if let Some(right) = &node.right {
                //     self.stack.push(right);
                // }
                // if let Some(left) = &node.left {
                //     self.stack.push(left);
                // }
                Some(ret)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    impl<T> Avl<T>
    where
        T: Ord + fmt::Debug,
    {
        pub fn iter_bfs(&self) -> IterBfs<'_, T> {
            let mut queue = VecDeque::new();
            if let Some(node) = &self.root {
                queue.push_back(&**node);
            }
            IterBfs { queue }
        }

        fn height2(p: &Option<Box<AvlNode<T>>>) -> isize {
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

        fn inorder_dfs<'a>(node: &'a Option<Box<AvlNode<T>>>, res: &mut Vec<&'a T>) {
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

    impl<T> fmt::Display for Avl<T>
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
                write!(f, "{to_print}")?;
            }
            write!(f, ")")
        }
    }

    pub struct IterBfs<'a, T: Ord> {
        queue: VecDeque<&'a AvlNode<T>>,
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

    // test case from:
    // https://stackoverflow.com/questions/3955680/how-to-check-if-my-avl-tree-implementation-is-correct

    #[test]
    fn test_avl() {
        let mut t1 = Avl::new();
        for i in [20, 4, 26, 3, 9, 15] {
            t1.insert(i);
        }
        assert_eq!(format!("{t1}"), "(9, 4, 20, 3, 15, 26)");
        // dbgt!(&t1);
        // println!("{}", t1);

        let mut t2 = Avl::new();
        for i in [20, 4, 26, 3, 9, 21, 30, 2, 7, 11, 15] {
            t2.insert(i);
        }
        assert_eq!(format!("{t2}"), "(9, 4, 20, 3, 7, 11, 26, 2, 15, 21, 30)");
        // dbgt!(&t2);
        // println!("{}", t2);

        let mut t3 = Avl::new();
        for i in [20, 4, 26, 3, 9, 8] {
            t3.insert(i);
        }
        assert_eq!(format!("{t3}"), "(9, 4, 20, 3, 8, 26)");
        // dbgt!(&t3);
        // println!("{}", t3);

        let mut t4 = Avl::new();
        for i in [20, 4, 26, 3, 9, 21, 30, 2, 7, 11, 8] {
            t4.insert(i);
        }
        assert_eq!(format!("{t4}"), "(9, 4, 20, 3, 7, 11, 26, 2, 8, 21, 30)");
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
        let mut t5 = Avl::new();
        for i in [2, 1, 4, 3, 5] {
            t5.insert(i);
        }
        assert_eq!(Some(1), t5.remove_by(|x| 1.cmp(x)));
        assert_eq!(format!("{t5}"), "(4, 2, 5, 3)");
        // dbgt!(&t5);
        // println!("{}", t5);

        let mut t6 = Avl::new();
        for i in [6, 2, 9, 1, 4, 8, 66, 3, 5, 7, 65, 67, 68] {
            t6.insert(i);
        }
        t6.remove(&1);
        assert_eq!(
            format!("{t6}"),
            "(6, 4, 9, 2, 5, 8, 66, 3, 7, 65, 67, 68)"
        );
        // dbgt!(&t6);
        // println!("{}", t6);

        let mut t7 = Avl::new();
        for i in [5, 2, 8, 1, 3, 7, 65, 4, 6, 9, 66, 67] {
            t7.insert(i);
        }
        t7.remove(&1);
        assert_eq!(format!("{t7}"), "(8, 5, 65, 3, 7, 9, 66, 2, 4, 6, 67)");
        // dbgt!(&t7);
        // println!("{}", t7);

        ////////////////////////////////////////////////////////////////////////
        // iter
        ////////////////////////////////////////////////////////////////////////
        assert_eq!(
            t7.iter_dfs().collect::<Vec<&i32>>(),
            [2, 3, 4, 5, 6, 7, 8, 9, 65, 66, 67]
                //[8, 5, 3, 2, 4, 7, 6, 65, 9, 66, 67]
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

        let mut t8 = Avl::new();
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

        let mut t9 = Avl::new();
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
        assert_eq!(None, t9.remove_by(|x| 9.cmp(&x.0)));
        assert!(!t9.insert((4, 5)));
        t9.pprint();
    }
}
