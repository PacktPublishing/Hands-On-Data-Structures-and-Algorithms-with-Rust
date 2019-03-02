use std::cell::{Ref, RefCell};
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    next: Vec<Link>,
    pub offset: u64,
    pub command: String,
}

impl Node {
    fn new(links: Vec<Link>, offset: u64, command: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            next: links,
            offset: offset,
            command: command,
        }))
    }
}

#[derive(Clone)]
pub struct BestTransactionLog {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

impl BestTransactionLog {
    pub fn new_empty(max_level: usize) -> BestTransactionLog {
        BestTransactionLog {
            max_level: max_level,
            head: None,
            tails: vec![None; max_level + 1],
            length: 0,
        }
    }

    fn get_level(&self) -> usize {
        let mut n = 0;
        // bool = p(true) = 0.5
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }

    pub fn append(&mut self, offset: u64, value: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level   // use the maximum level for the first node
        } else { 
            self.get_level() // determine the level by coin flips
        };

        let new = Node::new(vec![None; level], offset, value);

        // update the tails for each level
        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new.clone());
            }
            self.tails[i] = Some(new.clone());
        }

        // this is the first node in the list
        if self.head.is_none() {
            self.head = Some(new.clone());
        }
        self.length += 1;
    }

    pub fn find(&self, offset: u64) -> Option<String> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level;
                let node = head.clone();
                let mut result = None;
                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1;
                }
                let mut n = node;
                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref next) if next.borrow().offset <= offset => n = next.clone(),
                            _ => break
                        };
                    }
                    if n.borrow().offset == offset {
                        let tmp = n.borrow();
                        result = Some(tmp.command.clone());
                        break;
                    }
                }
                result
            }
            None => None,
        }
    }

    fn iter_level(&self, level: usize) -> ListIterator {
        ListIterator::new(self.head.clone(), level)
    }
}


impl IntoIterator for BestTransactionLog {
    type Item = (u64, String);
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head, 0)
    }
}

pub struct ListIterator {
    current: Option<Rc<RefCell<Node>>>,
    level: usize,
}

impl ListIterator {
    fn new(start_at: Option<Rc<RefCell<Node>>>, level: usize) -> ListIterator {
        ListIterator {
            current: start_at,
            level: level,
        }
    }
}

impl Iterator for ListIterator {
    type Item = (u64, String);

    fn next(&mut self) -> Option<(u64, String)> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some((current.offset, current.command.clone()));
                current.next[self.level].clone()
            },
            _ => None
        };
        result
    }
}


impl std::fmt::Debug for BestTransactionLog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.head {
            Some(ref _head) => {
                for level in (0..=self.max_level).rev() {
                    let _ = write!(f, "{}: ", level);
                    for n in self.iter_level(level) {
                        let _ = write!(f, "[{}] ", n.0);
                    }
                    let _ = writeln!(f, "");
                }
                Ok(())
            }
            None => write!(f, "The list is empty: []")
        }
    }
}
