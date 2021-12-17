pub trait Bisect<T> {
    fn bisect(&self, val:T) -> usize;
    fn bisect_left(&self, val: T) -> usize;
    fn bisect_right(&self, val: T) -> usize;
}

impl<T> Bisect<T> for [T]
where
    T: PartialOrd,
{
    fn bisect(&self, val: T) -> usize {
        let mut lo = 0;
        let mut hi = self.len(); // (len() -1, <=, mid -1) will overflow
        while lo < hi {
            let mid = lo + hi >> 1;
	    if self[mid] == val {
		return mid;
	    } else if self[mid] < val {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }

    fn bisect_left(&self, val: T) -> usize {
        let mut lo = 0;
        let mut hi = self.len();
        while lo < hi {
            let mid = lo + hi >> 1;
            if self[mid] < val {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }

    fn bisect_right(&self, val: T) -> usize {
        let mut lo = 0;
        let mut hi = self.len();
        while lo < hi {
            let mid = lo + hi >> 1;
            if self[mid] <= val {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }
}

#[test]
fn test_bisect() {
    let vec1 = vec![2, 2, 2, 3, 4, 5, 6, 6, 6, 6];
    assert_eq!(0, vec1.bisect_left(2));
    assert_eq!(3, vec1.bisect_right(2));
    assert_eq!(6, vec1.bisect_left(6));
    assert_eq!(10, vec1.bisect_right(6));
}
