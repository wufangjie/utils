#[derive(Debug)]
pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let rank = vec![0; n];
        Self { parent, rank }
    }

    pub fn find_set(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find_set(self.parent[i]);
        }
        self.parent[i]
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let si = self.find_set(i);
        let sj = self.find_set(j);
        if self.rank[si] > self.rank[sj] {
            self.parent[sj] = si;
        } else {
            self.parent[si] = sj;
            if self.rank[si] == self.rank[sj] {
                self.rank[sj] += 1;
            }
        }
    }

    pub fn is_same_component(&mut self, i: usize, j: usize) -> bool {
        self.find_set(i) == self.find_set(j)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_disjointset() {
        let dct: HashMap<char, usize> = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
            .into_iter()
            .enumerate()
            .map(|(i, c)| (c, i))
            .collect();
        let mut ds = DisjointSet::new(dct.len());
        for (c1, c2) in [
            ('b', 'd'),
            ('e', 'g'),
            ('a', 'c'),
            ('h', 'i'),
            ('a', 'b'),
            ('e', 'f'),
            ('b', 'c'),
        ] {
            ds.union(*dct.get(&c1).unwrap(), *dct.get(&c2).unwrap());
        }
        assert!(ds.is_same_component(0, 1));
        assert!(ds.is_same_component(1, 2));
        assert!(ds.is_same_component(2, 3));
        assert!(!ds.is_same_component(3, 4));
        assert!(ds.is_same_component(4, 5));
        assert!(ds.is_same_component(5, 6));
        assert!(!ds.is_same_component(6, 7));
        assert!(ds.is_same_component(7, 8));
        assert!(!ds.is_same_component(8, 9));
        assert_eq!(ds.parent, vec![3, 3, 3, 3, 6, 6, 6, 8, 8, 9]);
    }
}
