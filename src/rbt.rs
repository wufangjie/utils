//! red black tree
//!
//! version 0.1.1
//! https://github.com/wufangjie/utils/blob/main/src/rbt.rs
//!
//! we can add an additional field to RbtNode, to make a treemap,
//! but the best choice seems to use std::collections::BTreeMap,
//! which is more cache efficiency (modern computer architecture),
//! alse see https://www.zhihu.com/question/516912481
//!
//! I did not add iter_dfs, it is totally the same as avl's

use std::cmp::Ordering;
use std::fmt;
use std::mem;

#[derive(Debug)]
pub struct Rbt<T: Ord> {
    root: Option<Box<RbtNode<T>>>,
}

impl<T: Ord> Default for Rbt<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> Rbt<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}

/// impl: search, insert and remove
impl<T: Ord> Rbt<T> {
    pub fn search(&self, item: &T) -> bool {
        self.search_by(|x| item.cmp(x)).is_some()
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
    #[allow(unused_must_use)]
    pub fn insert(&mut self, item: T) -> bool {
        let (count, ret) = Self::insert_rec(&mut self.root, item);
        if count == 1 {
            self.root.as_mut().unwrap().color = Color::Black;
        }
        ret
    }

    pub fn remove(&mut self, item: &T) {
        self.remove_by(|x| item.cmp(x));
    }

    pub fn remove_by(&mut self, cmp: impl Fn(&T) -> Ordering) -> Option<T> {
        Self::remove_by_rec(&mut self.root, cmp).1
    }

    /// i8: {0, -1} means not decrease black node, decreased 1 black node
    fn remove_by_rec(
        node: &mut Option<Box<RbtNode<T>>>,
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
                        match inner.color {
                            Color::Black => (-1, Some(node.take().unwrap().data)),
                            Color::Red => (0, Some(node.take().unwrap().data)),
                        }
                    } else {
                        // this right must a leaf node with red color, black-red
                        let mut right = node.as_mut().unwrap().right.take();
                        mem::swap(node, &mut right);
                        Self::set_color(node, 0, Color::Black);
                        (0, Some(right.unwrap().data))
                    }
                } else {
                    let (mut delta, mut removed) = Self::remove_right_most_rec(&mut inner.left);
                    mem::swap(&mut inner.data, &mut removed.data);
                    delta = Self::backtrace_remove(node, -1, delta);
                    (delta, Some(removed.data))
                }
            }
            Ordering::Greater => {
                let (mut delta, ret) = Self::remove_by_rec(&mut node.as_mut().unwrap().right, cmp);
                delta = Self::backtrace_remove(node, 1, delta);
                (delta, ret)
            }
            Ordering::Less => {
                let (mut delta, ret) = Self::remove_by_rec(&mut node.as_mut().unwrap().left, cmp);
                delta = Self::backtrace_remove(node, -1, delta);
                (delta, ret)
            }
        }
    }

    fn remove_right_most_rec(node: &mut Option<Box<RbtNode<T>>>) -> (i8, Box<RbtNode<T>>) {
        if node.as_ref().unwrap().right.is_some() {
            let (mut delta, ret) = Self::remove_right_most_rec(&mut node.as_mut().unwrap().right);
            delta = Self::backtrace_remove(node, 1, delta);
            (delta, ret)
        } else {
            let mut left = node.as_mut().unwrap().left.take();
            mem::swap(node, &mut left);
            match left.as_ref().unwrap().color {
                Color::Black => {
                    if node.is_some() {
                        Self::set_color(node, 0, Color::Black);
                        (0, left.unwrap())
                    } else {
                        (-1, left.unwrap())
                    }
                }
                Color::Red => (0, left.unwrap()),
            }
        }
    }

    /// NOTE: other branch at least have a black node
    /// which means left(-1) or right(1) child being removed a node
    /// count means removed how many black node {0, -1}
    /// we only need to consider the non-remove branch's red node (3 levels)
    /// level1 means one of its child being removed a black node
    /// case1: level2 is red, rotate once to make it to other cases
    /// case2: level3 the outside node is red, rotate once
    /// case3: level3 the inside node is red, rotate twice
    /// case4: level1 is red, change it to black, then level2 to red
    /// case5: all of them are black, change level2 to red, then backtrace
    fn backtrace_remove(node: &mut Option<Box<RbtNode<T>>>, which: i8, count: i8) -> i8 {
        if count == 0 {
            0
        } else {
            let c0 = Self::get_color(node, 0);
            if let Color::Red = Self::get_color(node, -which) {
                // case1
                Self::rotate(node, which);
                Self::set_color(node, 0, Color::Black);
                Self::set_color(node, which, Color::Red);
                Self::backtrace_remove(node, which, -1) // recursive
            } else if let Color::Red = Self::get_color(node, -2 * which) {
                // case2
                Self::rotate(node, which);
                Self::set_color(node, 0, c0);
                Self::set_color(node, -1, Color::Black);
                Self::set_color(node, 1, Color::Black);
                0
            } else if let Color::Red = Self::get_color(Self::get_child(node, -which), which) {
                // case3
                Self::rotate(Self::get_child(node, -which), -which);
                Self::rotate(node, which);
                Self::set_color(node, 0, c0);
                Self::set_color(node, -1, Color::Black);
                Self::set_color(node, 1, Color::Black);
                0
            } else if let Color::Red = c0 {
                // case4
                Self::set_color(node, 0, Color::Black);
                Self::set_color(node, -which, Color::Red);
                0
            } else {
                // case5
                Self::set_color(node, -which, Color::Red);
                -1 // backtrace
            }
        }
    }

    /// i8 means the number of continuous red children, -1 means no need to check
    /// bool means insert succeed or not
    fn insert_rec(node: &mut Option<Box<RbtNode<T>>>, item: T) -> (i8, bool) {
        if node.is_none() {
            let mut leaf = Some(Box::new(RbtNode::new(item)));
            mem::swap(node, &mut leaf);
            return (1, true);
        }
        match item.cmp(&node.as_ref().unwrap().data) {
            Ordering::Equal => (0, false),
            Ordering::Greater => {
                let (mut count, succeed) =
                    Self::insert_rec(&mut node.as_mut().unwrap().right, item);
                count = Self::backtrace_insert(node, 1, count);
                (count, succeed)
            }
            Ordering::Less => {
                let (mut count, succeed) = Self::insert_rec(&mut node.as_mut().unwrap().left, item);
                count = Self::backtrace_insert(node, -1, count);
                (count, succeed)
            }
        }
    }

    /// this backtrace only process red-red case (child and its parent are both red)
    /// NOTE: current level1 is always black
    /// case1: other branch level2 is red, change level2 black, level1 red, then backtrace
    /// case2: inserting branch's level3 outside node are red, rotate once
    /// case3: inserting branch's level3 inside node are red, rotate twice
    /// why put case1 first, as this case, we can not rotate (will add it a red parent)
    fn backtrace_insert(node: &mut Option<Box<RbtNode<T>>>, which: i8, count: i8) -> i8 {
        match count {
            0 => 0,
            2 => {
                if let Color::Red = Self::get_color(node, -which) {
                    // case1
                    Self::set_color(node, 0, Color::Red);
                    Self::set_color(node, -1, Color::Black);
                    Self::set_color(node, 1, Color::Black);
                    1
                } else {
                    if let Color::Red = Self::get_color(node, 2 * which) {
                        // case2
                        Self::rotate(node, -which);
                    } else {
                        // case3
                        Self::rotate(Self::get_child(node, which), which);
                        Self::rotate(node, -which);
                    }
                    Self::set_color(node, 0, Color::Black);
                    Self::set_color(node, -which, Color::Red);
                    0
                }
            }
            x => match node.as_ref().unwrap().color {
                Color::Black => 0,
                Color::Red => x + 1,
            },
        }
    }

    /// rotate right without updating diff
    fn rotate_right(top: &mut Option<Box<RbtNode<T>>>) {
        let mut left = top.as_mut().unwrap().left.take();
        let lr = left.as_mut().unwrap().right.take();
        top.as_mut().unwrap().left = lr;
        mem::swap(&mut left, top);
        top.as_mut().unwrap().right = left;
    }

    /// rotate left without updating diff
    fn rotate_left(top: &mut Option<Box<RbtNode<T>>>) {
        let mut right = top.as_mut().unwrap().right.take();
        let rl = right.as_mut().unwrap().left.take();
        top.as_mut().unwrap().right = rl;
        mem::swap(&mut right, top);
        top.as_mut().unwrap().left = right;
    }

    /// rotate left or right, using which, used to simplify code
    /// which: -1 means rotate_left, 1 means rotate_right
    #[inline]
    fn rotate(top: &mut Option<Box<RbtNode<T>>>, which: i8) {
        match which {
            -1 => Self::rotate_left(top),
            1 => Self::rotate_right(top),
            _ => unreachable!(),
        }
    }

    /// which: -1 means left child, 1 means right child
    fn get_child(top: &mut Option<Box<RbtNode<T>>>, which: i8) -> &mut Option<Box<RbtNode<T>>> {
        match which {
            -1 => &mut top.as_mut().unwrap().left,
            1 => &mut top.as_mut().unwrap().right,
            _ => unreachable!(),
        }
    }

    /// which's possible values: {-2, -1, 0, 1, 2}
    /// -2: left child's left child
    /// -1: left child
    /// 0: current node
    /// 1: right child
    /// 2: right child's right child
    fn get_color(node: &Option<Box<RbtNode<T>>>, which: i8) -> Color {
        if node.is_none() {
            return Color::Black;
        }
        match which {
            0 => node.as_ref().unwrap().color,
            x if x > 0 => Self::get_color(&node.as_ref().unwrap().right, x - 1),
            x => Self::get_color(&node.as_ref().unwrap().left, x + 1),
        }
    }

    /// which's possible values: {-1, 0, 1} means to set left child, current node or right child
    fn set_color(node: &mut Option<Box<RbtNode<T>>>, which: i8, color: Color) {
        match which {
            0 => {
                if let Some(inner) = node {
                    inner.color = color
                }
            }
            -1 => Self::set_color(&mut node.as_mut().unwrap().left, 0, color),
            1 => Self::set_color(&mut node.as_mut().unwrap().right, 0, color),
            _ => unreachable!(),
        };
    }
}

#[derive(Clone, Copy)]
pub enum Color {
    Red,
    Black,
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "R"),
            Color::Black => write!(f, "B"),
        }
    }
}

#[derive(Debug)]
pub struct RbtNode<T> {
    data: T,
    left: Option<Box<RbtNode<T>>>,
    right: Option<Box<RbtNode<T>>>,
    color: Color,
}

impl<T> RbtNode<T>
where
    T: Ord,
{
    pub fn new(data: T) -> Self {
        RbtNode {
            data,
            left: None,
            right: None,
            color: Color::Red,
        }
    }

    pub fn new_black(data: T) -> Self {
        RbtNode {
            data,
            left: None,
            right: None,
            color: Color::Black,
        }
    }
}

impl<T> RbtNode<T>
where
    T: Ord + fmt::Debug,
{
    fn pprint_dfs(&self, indent: &str, cur: &str) {
        if let Some(left) = &self.left {
            left.pprint_dfs(&Self::next_indent(indent, cur, "┗"), "┏");
        }
        println!("{}{}━{:?}{:?}", indent, cur, &self.data, self.color);
        if let Some(right) = &self.right {
            right.pprint_dfs(&Self::next_indent(indent, cur, "┏"), "┗");
        }
    }

    fn next_indent(indent: &str, pre: &str, not_cur: &str) -> String {
        String::from(indent) + if pre != not_cur { "  " } else { "┃ " }
    }
}

impl<T> Rbt<T>
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

#[cfg(test)]
mod tests {
    use super::*;

    impl<T: Ord + fmt::Debug> Rbt<T> {
        fn inorder_dfs<'a>(node: &'a Option<Box<RbtNode<T>>>, res: &mut Vec<&'a T>) {
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

        fn is_rbt(&self) -> bool {
            if let Color::Red = Self::get_color(&self.root, 0) {
                false
            } else {
                //dbg!(Self::is_rbt_rec(&self.root, 0));
                Self::is_rbt_rec(&self.root, 0) != -1
            }
        }

        fn is_rbt_rec(node: &Option<Box<RbtNode<T>>>, mut count_black: isize) -> isize {
            if node.is_none() {
                return count_black;
            }
            let inner = node.as_ref().unwrap();
            if let Color::Red = inner.color {
                if let Color::Red = Self::get_color(&inner.left, 0) {
                    println!("Error: two red node {:?}", &inner.data);
                    return -1;
                };
                if let Color::Red = Self::get_color(&inner.right, 0) {
                    println!("Error: two red node {:?}", &inner.data);
                    return -1;
                };
            } else {
                count_black += 1;
            }
            let count_left = Self::is_rbt_rec(&inner.left, count_black);
            let count_right = Self::is_rbt_rec(&inner.left, count_black);
            if count_left != count_right {
                println!(
                    "Error: different black count: {}, {}",
                    count_left, count_right
                );
                -1
            } else {
                count_left
            }
        }
    }

    #[test]
    fn test_rbt() {
        // let mut t1 = Rbt::new();
        // for i in [20, 4, 26, 3, 9, 15] {
        //     //, 3, 9, 15
        //     t1.insert(i);
        // }
        // t1.pprint();

        let lst = vec![
            8, 94, 4, 50, 17, 15, 37, 1, 25, 42, 39, 13, 83, 32, 89, 24, 6, 70, 90, 22, 10, 11, 68,
            72, 49, 99, 45, 19, 38, 28, 63, 16, 77, 46, 65, 33, 34, 60, 53, 54, 40, 84, 2, 56, 57,
            44, 59, 92, 95, 41, 98, 97, 80, 29, 87, 18, 26, 67, 79, 88, 30, 20, 35, 81, 78, 55, 12,
            43, 85, 82, 0, 62, 96, 61, 71, 23, 9, 74, 27, 91, 69, 76, 52, 47, 64, 86, 75, 5, 7, 36,
            21, 93, 66, 31, 50, 17, 58, 14, 3, 51, 48, 73, // 50, 17 insert twice
        ];
        let mut t8 = Rbt::new();

        for i in lst.clone() {
            t8.insert(i);
        }
        //t8.pprint();
        assert!(!t8.insert(50));
        t8.assert_valid_bst();

        for i in lst {
            t8.remove(&i);
            //println!("removed {}", i);
            assert!(t8.is_rbt());
        }
        t8.assert_valid_bst();
        //t8.pprint();

        let a = &"hello world".to_string();
        let b = std::array::from_ref(&a);
        crate::dbgt!(&b);
        let c = &[a.clone()];
        crate::dbgt!(&c);
    }
}
