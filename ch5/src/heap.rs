use crate::MessageNotification;
use std::boxed::Box;
use std::mem;

pub struct MessageChecker {
    pub length: usize,
    heap: Vec<Box<MessageNotification>>,
}

impl MessageChecker {
    pub fn new_empty() -> MessageChecker {
        MessageChecker {
            length: 0,
            heap: vec![],
        }
    }

    fn swap(&mut self, pos1: usize, pos2: usize) {
        let m2 = self.heap[pos1 - 1].clone();
        self.heap[pos1 - 1] = mem::replace(&mut self.heap[pos2 - 1], m2);
    }

    fn has_more_messages(&self, pos1: usize, pos2: usize) -> bool {
        let a = &self.heap[pos1 - 1];
        let b = &self.heap[pos2 - 1];
        a.no_messages >= b.no_messages
    }

    pub fn add(&mut self, notification: MessageNotification) {
        self.heap.push(Box::new(notification));
        self.length = self.heap.len();

        if self.length > 1 {
            let mut i = self.length;
            while i / 2 > 0 && self.has_more_messages(i, i / 2) {
                self.swap(i, i / 2);
                i /= 2;
            }
        }
    }

    pub fn pop(&mut self) -> Option<MessageNotification> {
        if self.length > 0 {
            let elem = self.heap.swap_remove(0);
            self.length = self.heap.len();
            let mut i = 1;
            while i * 2 < self.length {
                let children = (i * 2, i * 2 + 1);
                i = if self.has_more_messages(children.0, children.1) {
                    if self.has_more_messages(children.0, i) {
                        self.swap(i, children.0);
                        children.0
                    } else {
                        break;
                    }
                } else {
                    if self.has_more_messages(children.1, i) {
                        self.swap(i, children.1);
                        children.1
                    } else {
                        break;
                    }
                }
            }
            Some(*elem)
        } else {
            None
        }
    }
}
