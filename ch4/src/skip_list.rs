use std::cell::{Ref, RefCell};
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
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
            tails: vec![None; max_level],
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
        let level = self.get_level() + 1;
        let new = Node::new(vec![None; level], offset, value);
        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let mut i = 0;
                let next = &mut old.borrow_mut().next;
                while i < level && i < next.len() {
                    next[i] = Some(new.clone());
                    i += 1
                }
            }
            self.tails[i] = Some(new.clone());
        }
        self.length += 1;
    }

    // pub fn find(&self, offset: u64) -> Option<String> {
    //     if let Some(ref head) = self.head {
    //         let mut i = self.max_level;
    //         let node = head.borrow();
    //         loop {
    //             if node.next[i].is_some() {
    //                 break;
    //             }
    //             i -= 1;
    //         }
    //         self.find_r(i, node, offset)
    //     } else {
    //         None
    //     }
    // }

    // fn find_r(&self, level: usize, start_node: Ref<Node>, offset: u64) -> Option<String> {
    //     let mut next = start_node;
    //     loop {
    //         let mut next = Some(next);
    //         match next {
    //             Some(ref _next) => {
    //                 if _next.offset > offset {
    //                     break;
    //                 }
    //                 next = _next.next[level];
    //             }
    //             None => break
    //         }
    //     }
    //     if offset == next.offset {
    //         Some(next.command.clone())
    //     } else {
    //         if level > 0 {
    //             self.find_r(level - 1, next, offset)
    //         } else {
    //             None
    //         }
    //     }
    // }

    // pub fn pop(&mut self) -> Option<String> {
    //     self.head.take().map(|head| {
    //         if let Some(next) = head.borrow_mut().next.take() {
    //             next.borrow_mut().prev = None;
    //             self.head = Some(next);
    //         } else {
    //             self.tail.take();
    //         }
    //         self.length -= 1;
    //         Rc::try_unwrap(head)
    //             .ok()
    //             .expect("Something is terribly wrong")
    //             .into_inner()
    //             .value
    //     })
    // }
}

impl std::fmt::Debug for SkipList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "BEHOLD THE SKIP LIST");

        // if let Some(ref head) = self.head {
        //     for lvl in 0..self.max_level {
        //         let mut next = head.as_mut();
        //         write!(f, "[{}]", lvl);
        //         loop {
        //             if let Some(ref _next) = next.next[lvl] {
        //                 write!(f, "{} -> ", next.offset);
        //                 next = _next.borrow();
        //             } else {
        //                 break;
        //             }
        //         }
        //         write!(f, "{} -> ", next.offset);
        //     }
        // }
        Ok(())
    }
}

