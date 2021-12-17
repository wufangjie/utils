//! A Min-BinaryHeap implementation.
//!
//! version 0.1.3
//! https://github.com/wufangjie/utils/blob/main/src/heap.rs
//!
//! NOTE: std::collections::BinaryHeap is a max heap,
//! as fast as this min heap implemention.

#[derive(Debug)]
pub struct Heap<T: PartialOrd> {
    data: Vec<T>,
    size: usize,
}

impl<T> Heap<T>
where
    T: PartialOrd,
{
    pub fn new() -> Heap<T> {
        Heap {
            data: vec![],
            size: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let ret = self.data.swap_remove(0);
            self.size -= 1;
            if self.size > 0 {
                self.heapify_downward(0);
            }
            Some(ret)
        }
    }

    pub fn push(&mut self, item: T) {
        self.size += 1;
        self.data.push(item);
        self.heapify_upward(self.size - 1);
    }

    pub fn pushpop(&mut self, mut item: T) -> T {
        if self.size > 0 && item > self.data[0] {
            std::mem::swap(&mut item, &mut self.data[0]);
            self.heapify_downward(0);
        }
        item
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    // pub fn peek_mut(&mut self) -> Option<&mut T> {
    // 	// TODO: directly get_mut may break heap
    //     self.data.get_mut(0)
    // }

    fn heapify_downward(&mut self, i: usize) {
        let j = (i + 1) << 1;
        if j < self.size && self.data[i] > self.data[j] {
            if self.data[j - 1] < self.data[j] {
                self.data.swap(i, j - 1);
                self.heapify_downward(j - 1);
            } else {
                self.data.swap(i, j);
                self.heapify_downward(j);
            }
        } else if j - 1 < self.size && self.data[i] > self.data[j - 1] {
            self.data.swap(i, j - 1);
            self.heapify_downward(j - 1);
        }
    }

    fn heapify_upward(&mut self, i: usize) {
        if i > 0 {
            let j = if (i & 1) == 1 {
                ((i + 1) >> 1) - 1
            } else {
                (i >> 1) - 1
            };
            if self.data[i] < self.data[j] {
                self.data.swap(i, j);
                self.heapify_upward(j);
            }
        }
    }
}

// #[test]
// fn test_heap() {
//     let mut heap = Heap::new();
//     heap.push(3);
//     heap.push(2);
//     heap.push(1);
//     *heap.peek_mut().unwrap() += 4;
//     dbg!(&heap);
// }
