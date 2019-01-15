use crate::IoTDevice;
use std::cell::{Ref, RefCell};
use std::cmp;
use std::mem;
use std::rc::Rc;

type BareTree = Rc<RefCell<Node>>;
type Tree = Option<BareTree>;

#[derive(Clone, Debug, PartialEq)]
enum Color {
    Red,
    Black,
}

#[derive(PartialEq)]
enum RBOperation {
    LeftNode,
    RightNode,
}

#[derive(PartialEq)]
enum Rotation {
    Left,
    Right,
}

struct Node {
    pub color: Color,
    pub dev: IoTDevice,
    pub parent: Tree,
    left: Tree,
    right: Tree,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.dev == other.dev
    }
}

impl Node {
    pub fn new(dev: IoTDevice) -> Tree {
        Some(Rc::new(RefCell::new(Node {
            color: Color::Red,
            dev: dev,
            parent: None,
            left: None,
            right: None,
        })))
    }
}

pub struct BetterDeviceRegistry {
    root: Tree,
    pub length: u64,
}

impl BetterDeviceRegistry {
    pub fn new_empty() -> BetterDeviceRegistry {
        BetterDeviceRegistry {
            root: None,
            length: 0,
        }
    }

    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        let new_tree = self.add_r(root, device);
        self.root = self.fix_tree(new_tree.1);
    }

    fn check(&self, a: &IoTDevice, b: &IoTDevice) -> RBOperation {
        if a.numerical_id <= b.numerical_id {
            RBOperation::LeftNode
        } else {
            RBOperation::RightNode
        }
    }

    fn add_r(&mut self, mut node: Tree, device: IoTDevice) -> (Tree, BareTree) {
        if let Some(n) = node.take() {
            let new: BareTree;
            let current_device = n.borrow().dev.clone();

            match self.check(&current_device, &device) {
                RBOperation::LeftNode => {
                    let left = n.borrow().left.clone();
                    let new_tree = self.add_r(left, device);
                    new = new_tree.1;
                    let new_tree = new_tree.0.unwrap();
                    new_tree.borrow_mut().parent = Some(n.clone());
                    n.borrow_mut().left = Some(new_tree);
                }

                RBOperation::RightNode => {
                    let right = n.borrow().right.clone();
                    let new_tree = self.add_r(right, device);
                    new = new_tree.1;
                    let new_tree = new_tree.0.unwrap();

                    new_tree.borrow_mut().parent = Some(n.clone());
                    n.borrow_mut().right = Some(new_tree);
                }
            }
            (Some(n), new)
        } else {
            let new = Node::new(device);
            (new.clone(), new.unwrap())
        }
    }

    pub fn is_a_valid_red_black_tree(&self) -> bool {
        let result = self.validate(&self.root, Color::Red, 0);
        let red_red = result.0;
        let black_height_min = result.1;
        let black_height_max = result.2;
        red_red == 0 && black_height_min == black_height_max
    }

    // red-red violations, min black-height, max-black-height
    fn validate(
        &self,
        node: &Tree,
        parent_color: Color,
        black_height: usize,
    ) -> (usize, usize, usize) {
        if let Some(n) = node {
            let n = n.borrow();
            let red_red = if parent_color == Color::Red && n.color == Color::Red {
                1
            } else {
                0
            };
            let black_height = black_height + match n.color {
                Color::Black => 1,
                _ => 0,
            };
            let l = self.validate(&n.left, n.color.clone(), black_height);
            let r = self.validate(&n.right, n.color.clone(), black_height);
            (red_red + l.0 + r.0, cmp::min(l.1, r.1), cmp::max(l.2, r.2))
        } else {
            (0, black_height, black_height)
        }
    }

    fn parent_color(&self, n: &BareTree) -> Color {
        n.borrow().parent.as_ref().unwrap().borrow().color.clone()
    }

    fn fix_tree(&mut self, inserted: BareTree) -> Tree {
        let mut not_root = inserted.borrow().parent.is_some();

        let root = if not_root {
            let mut parent_is_red = self.parent_color(&inserted) == Color::Red;
            let mut n = inserted.clone();
            while parent_is_red && not_root {
                if let Some(uncle) = self.uncle(n.clone()) {
                    let which = uncle.1;
                    let uncle = uncle.0;

                    match which {
                        RBOperation::LeftNode => {
                            // uncle is on the left
                            let mut parent = n.borrow().parent.as_ref().unwrap().clone();
                            if uncle.is_some()
                                && uncle.as_ref().unwrap().borrow().color == Color::Red
                            {
                                let uncle = uncle.unwrap();
                                parent.borrow_mut().color = Color::Black;
                                uncle.borrow_mut().color = Color::Black;
                                parent.borrow().parent.as_ref().unwrap().borrow_mut().color =
                                    Color::Red;

                                n = parent.borrow().parent.as_ref().unwrap().clone();
                            } else {
                                if self.check(&parent.borrow().dev, &n.borrow().dev)
                                    == RBOperation::LeftNode
                                {
                                    // do only if it's a right child
                                    let tmp = n.borrow().parent.as_ref().unwrap().clone();
                                    n = tmp;
                                    self.rotate(n.clone(), Rotation::Right);
                                    parent = n.borrow().parent.as_ref().unwrap().clone();
                                }
                                // until here. then for all black uncles
                                parent.borrow_mut().color = Color::Black;
                                parent.borrow().parent.as_ref().unwrap().borrow_mut().color =
                                    Color::Red;
                                let grandparent = n
                                    .borrow()
                                    .parent
                                    .as_ref()
                                    .unwrap()
                                    .borrow()
                                    .parent
                                    .as_ref()
                                    .unwrap()
                                    .clone();
                                self.rotate(grandparent, Rotation::Left);
                            }
                        }

                        RBOperation::RightNode => {
                            // uncle is on the right
                            let mut parent = n.borrow().parent.as_ref().unwrap().clone();

                            if uncle.is_some()
                                && uncle.as_ref().unwrap().borrow().color == Color::Red
                            {
                                let uncle = uncle.unwrap();

                                parent.borrow_mut().color = Color::Black;
                                uncle.borrow_mut().color = Color::Black;
                                parent.borrow().parent.as_ref().unwrap().borrow_mut().color =
                                    Color::Red;

                                n = parent.borrow().parent.as_ref().unwrap().clone();
                            } else {
                                if self.check(&parent.borrow().dev, &n.borrow().dev)
                                    == RBOperation::RightNode
                                {
                                    // do only if it's a right child
                                    let tmp = n.borrow().parent.as_ref().unwrap().clone();
                                    n = tmp;
                                    self.rotate(n.clone(), Rotation::Left);
                                    parent = n.borrow().parent.as_ref().unwrap().clone();
                                }
                                // until here. then for all black uncles
                                parent.borrow_mut().color = Color::Black;
                                parent.borrow().parent.as_ref().unwrap().borrow_mut().color =
                                    Color::Red;
                                let grandparent = n
                                    .borrow()
                                    .parent
                                    .as_ref()
                                    .unwrap()
                                    .borrow()
                                    .parent
                                    .as_ref()
                                    .unwrap()
                                    .clone();
                                self.rotate(grandparent, Rotation::Right);
                            }
                        }
                    }
                } else {
                    break;
                }

                not_root = n.borrow().parent.is_some();
                if not_root {
                    parent_is_red = self.parent_color(&n) == Color::Red;
                }
            }
            while n.borrow().parent.is_some() {
                let t = n.borrow().parent.as_ref().unwrap().clone();
                n = t;
            }
            Some(n)
        } else {
            Some(inserted)
        };
        root.map(|r| {
            r.borrow_mut().color = Color::Black;
            r
        })
    }

    fn rotate(&self, node: BareTree, direction: Rotation) {
        match direction {
            Rotation::Right => {
                let x = node;
                let y = x.borrow().left.clone();
                x.borrow_mut().left = match y {
                    Some(ref y) => y.borrow().right.clone(),
                    _ => None,
                };

                if y.is_some() {
                    y.as_ref().unwrap().borrow_mut().parent = x.borrow().parent.clone();
                    if y.as_ref().unwrap().borrow().right.is_some() {
                        let r = y.as_ref().unwrap().borrow().right.clone();
                        r.unwrap().borrow_mut().parent = Some(x.clone());
                    }
                }

                if let Some(ref parent) = x.borrow().parent {
                    let insert_direction = self.check(&parent.borrow().dev, &x.borrow().dev);
                    match insert_direction {
                        RBOperation::RightNode => parent.borrow_mut().right = y.clone(),
                        RBOperation::LeftNode => parent.borrow_mut().left = y.clone(),
                    }
                } else {
                    y.as_ref().unwrap().borrow_mut().parent = None;
                }
                y.as_ref().unwrap().borrow_mut().right = Some(x.clone());
                x.borrow_mut().parent = y.clone();
            }
            Rotation::Left => {
                let x = node;
                let y = x.borrow().right.clone();
                x.borrow_mut().right = match y {
                    Some(ref y) => y.borrow().left.clone(),
                    _ => None,
                };

                if y.is_some() {
                    y.as_ref().unwrap().borrow_mut().parent = x.borrow().parent.clone();

                    if y.as_ref().unwrap().borrow().left.is_some() {
                        let l = y.as_ref().unwrap().borrow().left.clone();
                        l.unwrap().borrow_mut().parent = Some(x.clone());
                    }
                }

                if let Some(ref parent) = x.borrow().parent {
                    let insert_direction = self.check(&parent.borrow().dev, &x.borrow().dev);

                    match insert_direction {
                        RBOperation::LeftNode => parent.borrow_mut().left = y.clone(),
                        RBOperation::RightNode => parent.borrow_mut().right = y.clone(),
                    }
                } else {
                    y.as_ref().unwrap().borrow_mut().parent = None;
                }
                y.as_ref().unwrap().borrow_mut().left = Some(x.clone());
                x.borrow_mut().parent = y.clone();
            }
        }
    }

    fn uncle(&self, tree: BareTree) -> Option<(Tree, RBOperation)> {
        let current = tree.borrow();

        if let Some(ref parent) = current.parent {
            let parent = parent.borrow();

            if let Some(ref grandparent) = parent.parent {
                let grandparent = grandparent.borrow();

                match self.check(&grandparent.dev, &parent.dev) {
                    RBOperation::LeftNode => {
                        Some((grandparent.right.clone(), RBOperation::RightNode))
                    }
                    RBOperation::RightNode => {
                        Some((grandparent.left.clone(), RBOperation::LeftNode))
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn find(&self, numerical_id: u64) -> Option<IoTDevice> {
        self.find_r(
            &self.root,
            &IoTDevice::new(numerical_id, "".to_owned(), "".to_owned()),
        )
    }

    fn find_r(&self, node: &Tree, dev: &IoTDevice) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                let n = n.borrow();
                if n.dev.numerical_id == dev.numerical_id {
                    Some(n.dev.clone())
                } else {
                    match self.check(&n.dev, &dev) {
                        RBOperation::LeftNode => self.find_r(&n.left, dev),
                        RBOperation::RightNode => self.find_r(&n.right, dev),
                    }
                }
            }
            _ => None,
        }
    }

    pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
        self.walk_in_order(&self.root, &callback);
    }

    fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice) -> ()) {
        if let Some(n) = node {
            let n = n.borrow();

            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
        }
    }
}
