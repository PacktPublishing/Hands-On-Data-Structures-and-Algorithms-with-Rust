use std::boxed::Box;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
            prev: None,
        }))
    }
}


#[derive(Debug, Clone)]
pub struct DoublyLinkedList {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl DoublyLinkedList {
    pub fn new_empty() -> DoublyLinkedList {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }
}

impl DoublyLinkedList {
    pub fn append(&mut self, value: String) {
        let new = Node::new(value);

        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
}
