//! A Min-BinaryHeap implementation.
//!
//! version 0.1.7
//! https://github.com/wufangjie/utils/blob/main/src/heap.rs
//!
//! NOTE: std::collections::BinaryHeap is a max heap,
//! as fast as this min heap implemention.

#[derive(Debug)]
pub struct Heap<T: PartialOrd> {
    data: Vec<T>,
}

impl<T: PartialOrd> Default for Heap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Heap<T>
where
    T: PartialOrd,
{
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn with_capacity(capacity: usize) -> Heap<T> {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let ret = self.data.swap_remove(0);
            if !self.is_empty() {
                self.heapify_downward(0);
            }
            Some(ret)
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.heapify_upward(self.len() - 1);
    }

    /// push then pop, TODO: do we need poppush?
    pub fn pushpop(&mut self, mut item: T) -> T {
        if !self.is_empty() && item > self.data[0] {
            std::mem::swap(&mut item, &mut self.data[0]);
            self.heapify_downward(0);
        }
        item
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    // pub fn peek_mut(&mut self) -> Option<&mut T> {
    // 	// TODO: directly get_mut may break the heap
    //     self.data.get_mut(0)
    // }

    fn heapify_downward(&mut self, mut i: usize) {
        let n = self.len();
        loop {
            let j = (i + 1) << 1;
            if j < n && self.data[i] > self.data[j] {
                if self.data[j - 1] < self.data[j] {
                    self.data.swap(i, j - 1);
                    i = j - 1;
                } else {
                    self.data.swap(i, j);
                    i = j;
                }
            } else if j - 1 < n && self.data[i] > self.data[j - 1] {
                self.data.swap(i, j - 1);
                i = j - 1;
            } else {
                return;
            }
        }
    }

    fn heapify_upward(&mut self, mut i: usize) {
        while i > 0 {
            let j = (i - 1) >> 1;
            if self.data[i] < self.data[j] {
                self.data.swap(i, j);
                i = j;
            } else {
                return;
            }
        }
    }

    // fn heapify_downward(&mut self, i: usize) {
    //     let j = (i + 1) << 1;
    //     if j < self.len() && self.data[i] > self.data[j] {
    //         if self.data[j - 1] < self.data[j] {
    //             self.data.swap(i, j - 1);
    //             self.heapify_downward(j - 1);
    //         } else {
    //             self.data.swap(i, j);
    //             self.heapify_downward(j);
    //         }
    //     } else if j - 1 < self.len() && self.data[i] > self.data[j - 1] {
    //         self.data.swap(i, j - 1);
    //         self.heapify_downward(j - 1);
    //     }
    // }

    // fn heapify_upward(&mut self, i: usize) {
    //     if i > 0 {
    //         let j = (i - 1) >> 1;
    //         if self.data[i] < self.data[j] {
    //             self.data.swap(i, j);
    //             self.heapify_upward(j);
    //         }
    //     }
    // }

    pub fn into_inner(self) -> Vec<T> {
	self.data
    }
}

impl<T: PartialOrd> From<Vec<T>> for Heap<T> {
    fn from(data: Vec<T>) -> Self {
        let mut res = Self { data };
        for i in (0..res.len() >> 1).rev() {
            res.heapify_downward(i);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_a_min_heap<T: PartialOrd + std::fmt::Display>(heap: &Heap<T>) {
        if heap.len() > 1 {
            let is_even = (heap.len() & 1) == 0;
            let mut n2 = heap.len() >> 1;
            if is_even {
                n2 -= 1;
            }
            for i in 0..n2 {
                println!(
                    "{}, {}, {}",
                    heap.data[i],
                    heap.data[2 * i + 1],
                    heap.data[2 * i + 2]
                );
                assert!(heap.data[i] <= heap.data[2 * i + 1]);
                assert!(heap.data[i] <= heap.data[2 * i + 2]);
            }
            if is_even {
                let i = n2;
                println!("{}, {}", heap.data[i], heap.data[2 * i + 1]);
                assert!(heap.data[i] <= heap.data[2 * i + 1]);
            }
        }
        println!();
    }

    #[test]
    fn test_heap() {
        let heap = Heap::from(vec![4, 9, 7, 3, 1, 8, 6, 0, 5, 2, 0]);
        is_a_min_heap(&heap);

        let heap = Heap::from(Vec::<i32>::new());
        is_a_min_heap(&heap);

        let heap = Heap::from(vec![1]);
        is_a_min_heap(&heap);

        //let mut heap = Heap::new();
        let mut heap = Heap::with_capacity(12);
        for i in &[11, 10, 4, 9, 7, 3, 1, 8, 6, 0, 5, 2] {
            heap.push(i);
            //println!("{:p}", &heap.data[0] as *const _);
        }
        is_a_min_heap(&heap);
    }
}
