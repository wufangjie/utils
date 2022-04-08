// https://leetcode.com/problems/range-sum-query-mutable/
// only implement add (mul, max, min (the last two will be different at updating))

#[derive(Debug)]
pub struct SegmentTree<T> {
    inter: Vec<T>,
    leafs: Vec<T>,
    n: usize,
}

impl<T> SegmentTree<T>
where
    T: std::fmt::Debug
        + Copy
        + Default
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::AddAssign,
{
    pub fn empty(n: usize) -> Self {
        let leafs = vec![Default::default(); n];
        let inter = vec![Default::default(); log2n(n - 1).max(2) - 1];
        Self { inter, leafs, n }
    }

    pub fn new(leafs: Vec<T>) -> Self {
        let n = leafs.len();
        let mut inter = vec![Default::default(); log2n(n - 1).max(2) - 1];
        Self::_new(&mut inter, &leafs, 0, n - 1, 0);
        Self { inter, leafs, n }
    }

    // pub fn new(leafs: Vec<T>) -> Self {
    //     let n = leafs.len();
    //     let mut inter = vec![Default::default(); log2n(n - 1).max(2) - 1];
    //     let mut stack = vec![(0, n - 1, 0)];
    //     while let Some((lo, hi, i)) = stack.pop() {
    //         match hi - lo {
    //             0 => Self::add_upward(&mut inter, leafs[lo], i),
    //             1 => Self::add_upward(&mut inter, leafs[lo] + leafs[hi], i),
    //             _ => {
    //                 let mid = lo + hi >> 1;
    //                 let i2 = i + 1 << 1;
    //                 stack.push((lo, mid, i2 - 1));
    //                 stack.push((mid + 1, hi, i2));
    //             }
    //         }
    //     }
    //     Self { inter, leafs, n }
    // }

    pub fn query(&self, min: usize, max: usize) -> T {
        let mut acc = Default::default();
        let mut stack = vec![(0, self.n - 1, 0)];
        while let Some((lo, hi, i)) = stack.pop() {
            if min > hi || max < lo {
                continue;
            } else if lo == hi {
                acc += self.leafs[lo];
            } else if min <= lo && max >= hi {
                acc += self.inter[i]
            } else {
                let mid = (lo + hi) >> 1;
                let i2 = (i + 1) << 1;
                stack.push((lo, mid, i2 - 1));
                stack.push((mid + 1, hi, i2));
            }
        }
        acc
    }

    pub fn update(&mut self, idx: usize, val: T) {
        let diff = val - self.leafs[idx];
        self.update_by_diff(idx, diff);
    }

    #[inline]
    pub fn update_by_diff(&mut self, idx: usize, diff: T) {
        self.leafs[idx] += diff;
        let mut i = 0;
        let mut lo = 0;
        let mut hi = self.n - 1;
        while hi - lo > 0 {
            self.inter[i] += diff;
            let mid = (lo + hi) >> 1;
            if idx > mid {
                lo = mid + 1;
                i = (i + 1) << 1;
            } else {
                hi = mid;
                i = ((i + 1) << 1) - 1;
            }
        }
    }

    fn _new(inter: &mut Vec<T>, leafs: &[T], lo: usize, hi: usize, i: usize) -> T {
        match hi - lo {
            0 => inter[i] = leafs[lo],
            1 => inter[i] = leafs[lo] + leafs[hi],
            _ => {
                let mid = (lo + hi) >> 1;
                let i2 = (i + 1) << 1;
                inter[i] = Self::_new(inter, leafs, lo, mid, i2 - 1)
                    + Self::_new(inter, leafs, mid + 1, hi, i2);
            }
        }
        inter[i]
    }

    // #[inline]
    // fn add_upward(inter: &mut Vec<T>, val: T, mut i: usize) {
    //     while i > 0 {
    //         inter[i] += val;
    //         i -= 1;
    //         i >>= 1;
    //     }
    //     inter[0] += val;
    // }
}

#[inline]
fn log2n(mut n: usize) -> usize {
    let mut res = 1;
    while n > 0 {
        n >>= 1;
        res <<= 1;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_segmenttree() {
        let mut st = SegmentTree::new(vec![1, 3, 5, 7, 9, 11]);
        //dbg!(&st);
        assert_eq!(36, st.query(0, 5));
        assert_eq!(9, st.query(0, 2));
        assert_eq!(15, st.query(1, 3));
        assert_eq!(32, st.query(2, 5));

        st.update(1, 9);
        //dbg!(&st);
        assert_eq!(42, st.query(0, 5));
        assert_eq!(15, st.query(0, 2));
        assert_eq!(21, st.query(1, 3));
        assert_eq!(32, st.query(2, 5));
    }
}
