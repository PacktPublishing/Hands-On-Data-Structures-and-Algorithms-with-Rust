use std::boxed::Box;
use std::cell::RefCell;
use std::collections::BTreeMap;

pub type NetworkDeviceStore = TrieSet<u8>;
type Link<K> = Box<Node<K>>;

struct Node<K>
where
    K: PartialEq + Clone + Ord,
{
    pub key: K,
    next: BTreeMap<K, Link<K>>,
    ends_here: bool,
}

impl<K> Node<K>
where
    K: PartialEq + Ord + Clone,
{
    pub fn new(key: K, ends_here: bool) -> Link<K> {
        Box::new(Node {
            key: key,
            next: BTreeMap::new(),
            ends_here: ends_here,
        })
    }
}

impl<K> PartialEq for Node<K>
where
    K: PartialEq + Clone + Ord,
{
    fn eq(&self, other: &Node<K>) -> bool {
        self.key == other.key
    }
}

pub struct TrieSet<K>
where
    K: PartialEq + Clone + Ord,
{
    pub length: u64,
    root: BTreeMap<K, Link<K>>,
}

impl<K> TrieSet<K>
where
    K: PartialEq + Clone + Ord,
{
    pub fn new_empty() -> TrieSet<K> {
        TrieSet {
            length: 0,
            root: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, elements: &[K]) {
        let mut path = elements.into_iter();

        if let Some(start) = path.next() {
            let mut n = self
                .root
                .entry(start.clone())
                .or_insert(Node::new(start.clone(), false));
            for c in path {
                let tmp = n
                    .next
                    .entry(c.clone())
                    .or_insert(Node::new(c.clone(), false));
                n = tmp;
            }
            if !n.ends_here {
                self.length += 1;
            }
            n.ends_here = true;
        }
    }

    pub fn contains(&self, key: &[K]) -> bool {
        let mut path = key.into_iter();

        if let Some(start) = path.next() {
            self.root.get(&start).map_or(false, |mut n| {
                for c in path {
                    match n.next.get(&c) {
                        Some(ref tmp) => n = tmp,
                        None => break,
                    }
                }
                n.ends_here
            })
        } else {
            false
        }
    }

    pub fn difference(self, other: TrieSet<K>) -> TrieSet<K> {
        let new = RefCell::new(TrieSet::new_empty());
        self.walk(|k| {
            if !other.contains(k) {
                new.borrow_mut().insert(k)
            }
        });
        new.into_inner()
    }

    pub fn union(self, other: TrieSet<K>) -> TrieSet<K> {
        let new = RefCell::new(TrieSet::new_empty());
        self.walk(|k| new.borrow_mut().insert(k));
        other.walk(|k| new.borrow_mut().insert(k));
        new.into_inner()
    }

    pub fn intersection(self, other: TrieSet<K>) -> TrieSet<K> {
        let new = RefCell::new(TrieSet::new_empty());
        if self.length < other.length {
            self.walk(|k| {
                if other.contains(k) {
                    new.borrow_mut().insert(k)
                }
            });
        } else {
            other.walk(|k| {
                if self.contains(k) {
                    new.borrow_mut().insert(k)
                }
            });
        }
        new.into_inner()
    }

    pub fn walk(&self, callback: impl Fn(&[K]) -> ()) {
        for r in self.root.values() {
            self.walk_r(&r, &vec![], &callback);
        }
    }

    fn walk_r(&self, node: &Link<K>, value: &Vec<K>, callback: &impl Fn(&[K]) -> ()) {
        let mut v = value.clone();
        v.push(node.key.clone());
        for n in node.next.values() {
            self.walk_r(&n, &v, callback);
        }
        if node.ends_here {
            callback(&v);
        }
    }
    pub fn into_iter(self) -> SetIterator<K> {
        let v: RefCell<Vec<Vec<K>>> = RefCell::new(vec![]);
        self.walk(|n| v.borrow_mut().push(n.to_vec()));
        SetIterator::new(v.into_inner(), 0)
    }
}

pub struct SetIterator<K>
where
    K: PartialEq + Clone + Ord,
{
    data: Vec<Vec<K>>,
    last_index: usize,
}

impl<K> SetIterator<K>
where
    K: PartialEq + Clone + Ord,
{
    fn new(data: Vec<Vec<K>>, start_at: usize) -> SetIterator<K> {
        SetIterator {
            data: data,
            last_index: start_at,
        }
    }
}

impl<K> Iterator for SetIterator<K>
where
    K: PartialEq + Clone + Ord,
{
    type Item = Vec<K>;

    fn next(&mut self) -> Option<Vec<K>> {
        let result = self.data.get(self.last_index);
        self.last_index += 1;
        result.cloned()
    }
}
