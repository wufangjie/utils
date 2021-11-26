// TODO: compare with std::collections::BinaryHeap

#[derive(Debug)]
pub struct Heap<T: PartialOrd> {
    size: usize,
    data: Vec<T>,
}

impl<T> Heap<T>
where
    T: PartialOrd,
{
    pub fn new() -> Heap<T> {
        Heap {
            size: 0,
            data: vec![],
        }
    }

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
        }
        if j - 1 < self.size && self.data[i] > self.data[j - 1] {
            self.data.swap(i, j - 1);
            self.heapify_downward(j - 1);
        }
    }

    pub fn push(&mut self, item: T) {
        self.size += 1;
        self.data.push(item);
        self.heapify_upward(self.size - 1);
    }

    fn heapify_upward(&mut self, i: usize) {
        if i == 0 {
            return;
        }
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
