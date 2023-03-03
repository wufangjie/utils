//! A Doubly Linked List implementation.
//!
//! version 0.1.1
//! https://github.com/wufangjie/utils/blob/main/src/linkedlist.rs
//!
//! Forward list share the ownership, backward list is just raw pointer
//! Only used two `unsafe` to save loop time

use std::cmp::Ordering;
use std::fmt;
use std::ptr;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>, // Box own the data they point to
    tail: *mut ListNode<T>,
    len: usize,
}

#[derive(Debug)]
pub struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
    prev: *mut ListNode<T>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push_back_node(&mut self, mut node: Box<ListNode<T>>) {
        let p = (*node).as_mut_ptr();
        if self.len == 0 {
            self.head = Some(node);
        } else {
            node.prev = self.tail;
            unsafe { (*self.tail).next = Some(node) }
        }
        self.tail = p;
        self.len += 1;
    }

    pub fn push_front_node(&mut self, mut node: Box<ListNode<T>>) {
        if self.len == 0 {
            self.tail = (*node).as_mut_ptr();
        } else {
            if let Some(first) = &mut self.head {
                first.prev = (*node).as_mut_ptr();
            }
            node.next = self.head.take();
        }
        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop_back_node(&mut self) -> Option<Box<ListNode<T>>> {
        let mut ret = None::<Box<ListNode<T>>>;
        if self.len > 0 {
            if self.len == 1 {
                std::mem::swap(&mut ret, &mut self.head);
                self.tail = ptr::null_mut();
            } else {
                let pre; // TODO: immutable pre can have an &mut pre.next
                unsafe {
                    pre = &mut *(*self.tail).prev; // not null
                }
                std::mem::swap(&mut ret, &mut pre.next);
                self.tail = pre.as_mut_ptr();
                if let Some(node) = &mut ret {
                    node.prev = ptr::null_mut(); // need this?
                }
            }
            self.len -= 1;
        }
        ret
    }

    pub fn pop_front_node(&mut self) -> Option<Box<ListNode<T>>> {
        let mut ret = None::<Box<ListNode<T>>>;
        if self.len > 0 {
            std::mem::swap(&mut ret, &mut self.head);
            if let Some(node) = &mut ret {
                std::mem::swap(&mut self.head, &mut node.next);
            }
            if let Some(node) = &mut self.head {
                node.prev = ptr::null_mut(); // need this?
            } else {
                self.tail = ptr::null_mut();
            }
            self.len -= 1;
        }
        ret
    }

    pub fn push_back(&mut self, v: T) {
        self.push_back_node(Box::new(ListNode::new(v))); // do not use box keyword
    }

    pub fn push_front(&mut self, v: T) {
        self.push_front_node(Box::new(ListNode::new(v)));
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(|node| node.data)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|node| node.data)
    }

    pub fn remove_node(p: &mut Option<Box<ListNode<T>>>) -> Option<T> {
        // NOTE: this is not a method, so we can not modify self.len here
        let mut temp = None::<Box<ListNode<T>>>;
        std::mem::swap(&mut temp, p);
        if let Some(to_remove) = &mut temp {
            std::mem::swap(p, &mut to_remove.next);
            if let Some(node) = p {
                node.prev = to_remove.prev;
            }
            to_remove.prev = ptr::null_mut(); // need this?
        }
        temp.map(|node| node.data)
    }

    pub fn remove_at(&mut self, i: usize) -> Option<T> {
        match i.cmp(&(self.len - 1)) {
            Ordering::Greater => None,
            Ordering::Equal => self.pop_back(),
            Ordering::Less => {
                let mut p = &mut self.head;
                for _ in 0..i {
                    if let Some(node) = p {
                        p = &mut node.next;
                    }
                }
                self.len -= 1;
                Self::remove_node(p)
            }
        }

        // if i > self.len - 1 {
        //     None
        // } else if i == self.len - 1 {
        //     self.pop_back()
        // } else {
        //     let mut p = &mut self.head;
        //     for _ in 0..i {
        //         if let Some(node) = p {
        //             p = &mut node.next;
        //         }
        //     }
        //     self.len -= 1;
        //     Self::remove_node(p)
        // }
    }

    pub fn remove_item(&mut self, item: T)
    where
        T: PartialEq,
    {
        let mut p = &mut self.head;
        while let Some(node) = p {
            if let Some(next) = &node.next {
                if next.data == item {
                    self.len -= 1;
                    Self::remove_node(&mut node.next);
                    break;
                }
            }
            p = &mut node.next;
        }
    }

    // pub fn from_iter<I>(iter: I) -> Self
    // where
    //     I: Iterator<Item = T>,
    // {
    //     let mut lst = LinkedList::new();
    //     for item in iter {
    //         lst.push_back(item);
    //     }
    //     lst
    // }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { head: &self.head }
    }

    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq,
    {
        self.iter().any(|v| v == x)
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut lst = Self::new();
        for item in iter {
            lst.push_back(item);
        }
        lst
    }
}

impl<T> fmt::Display for LinkedList<T>
where
    T: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        let mut is_first_time = true;
        let mut head = &self.head;
        while let Some(p) = head {
            if is_first_time {
                is_first_time = false
            } else {
                write!(f, " -> ")?;
            }
            write!(f, "{}", p.data)?;
            head = &p.next;
        }
        write!(f, ")")
    }
}

impl<T> ListNode<T> {
    pub fn new(item: T) -> Self {
        ListNode {
            data: item,
            next: None,
            prev: ptr::null_mut(),
        }
    }

    pub fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    pub fn as_mut_ptr(&self) -> *mut Self {
        // TODO: Does this unsafe?
        self as *const Self as usize as *mut Self
    }
}

pub struct Iter<'a, T> {
    head: &'a Option<Box<ListNode<T>>>,
    //marker: PhantomData<&'a ListNode<T>>, // NOTE: just for lifetime
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head {
            None => None,
            Some(node) => {
                self.head = &node.next;
                Some(&node.data)
            }
        }
    }
}

#[test]
fn test_linkedlist() {
    let mut ll = LinkedList::from_iter(vec![9, 2, 3, 4, 5, 6].into_iter());
    // let mut ll = LinkedList::new();
    // for v in [4, 5, 6].into_iter() {
    //     ll.push_back(v);
    // }
    // for v in [3, 2, 9].into_iter() {
    //     ll.push_front(v);
    // }

    let mut stack = vec![ll.pop_back(), ll.pop_front(), ll.pop_back()];
    // let mut stack = vec![];
    // stack.push(ll.pop_back());
    // stack.push(ll.pop_front());
    // stack.push(ll.pop_back());
    ll.push_back(42);
    ll.push_front(142);
    ll.push_front(12);
    stack.push(ll.pop_front());

    // [6, 9, 5, 12] + [142, 2, 3, 4, 42]
    while let Some(Some(v)) = stack.pop() {
        ll.push_front(v);
    }
    assert_eq!(Some(5), ll.remove_at(2));
    assert_eq!(None, ll.remove_at(8));
    ll.remove_item(3);
    ll.remove_item(33);
    assert_eq!(7, ll.len());

    assert_eq!("(6 -> 9 -> 12 -> 142 -> 2 -> 4 -> 42)", format!("{ll}"));
    assert!(ll.contains(&9));
    assert!(!ll.contains(&5));
    // assert!(ll.contains(&142));
    // assert!(!ll.contains(&3));
    // assert!(ll.contains(&42));
}
