/// A special case of trie: digit number

struct Trie {
    lst: [Option<Box<Trie>>; 10],
    // end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            lst: [None, None, None, None, None, None, None, None, None, None],
        }
    }

    pub fn from_iter(key: &[usize]) -> Self {
        let mut sub = Self::new();
        let mut i = key.len();
        while i > 0 {
            i -= 1;
            let mut trie = Self::new();
            trie.lst[key[i]] = Some(Box::new(sub));
            sub = trie;
        }
        sub
    }

    pub fn contains(&self, key: &[usize]) -> bool {
        let mut trie = self;
        for k in key {
            match &trie.lst[*k] {
                None => return false,
                Some(sub) => trie = sub,
            }
        }
        true
    }

    pub fn insert(&mut self, key: &[usize]) {
        let mut trie = self;
        let mut i = 0;
        for k in key {
            i += 1;
	    // NOTE: do not use match, which will destructure
            if trie.lst[*k].is_none() {
                trie.lst[*k] = Some(Box::new(Self::from_iter(&key[i..])));
                return;
            }
            trie = &mut *trie.lst[*k].as_mut().unwrap();
        }
    }
}
