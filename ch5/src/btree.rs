use crate::IoTDevice;
use std::cmp;
use std::collections::HashMap;
use std::mem;

type Tree = Box<Node>;
type KeyType = u64;

type Data = (Option<IoTDevice>, Option<Tree>);

#[derive(Clone, Debug)]
struct Node {
    devices: HashMap<KeyType, Data>,
    pub is_leaf: bool,
}

impl Node {
    pub fn new_leaf() -> Tree {
        Node::new(true)
    }

    pub fn new_regular() -> Tree {
        Node::new(false)
    }

    fn new(is_leaf: bool) -> Tree {
        let mut devices = HashMap::new();
        devices.insert(KeyType::min_value(), (None, None));
        Box::new(Node {
            devices: devices,
            is_leaf: is_leaf,
        })
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn split(&mut self) -> (IoTDevice, Tree) {
        let mut sibling = Node::new(self.is_leaf);

        let no_of_devices = self.len();
        let split_at = no_of_devices / 2usize;

        let mut ordered_keys: Vec<KeyType> = self.devices.keys().map(|k| *k).collect();
        ordered_keys.sort();

        let (dev, node) = self.devices.remove(&ordered_keys[split_at]).unwrap();
        ordered_keys.remove(split_at);

        for (key, value) in ordered_keys
            .iter()
            .enumerate()
            .filter(|(i, _)| i >= &split_at)
            .filter_map(|(_, k)| self.devices.remove_entry(k))
        {
            sibling.add_key(key, value);
        }

        sibling.add_key(KeyType::min_value(), (None, node));
        (dev.unwrap(), sibling)
    }

    pub fn add_key(&mut self, key: KeyType, value: Data) -> bool {
        self.devices.insert(key, value).is_none()
    }

    pub fn remove_key(&mut self, id: KeyType) -> Option<(KeyType, Data)> {
        let key = self.find_closest_key(id);
        self.devices.remove_entry(&key)
    }

    pub fn find_closest_key(&self, id: KeyType) -> KeyType {
        let mut ordered_keys: Vec<KeyType> = self.devices.keys().map(|k| *k).collect();
        ordered_keys.sort();
        ordered_keys
            .iter()
            .fold(KeyType::min_value(), |last, current| {
                if current <= &id {
                    *current
                } else {
                    last
                }
            })
    }

    fn print(&self, label: &str) {
        let devs: Vec<&u64> = self.devices.keys().collect();
        println!("{}: {:?}", label, devs);
    }
}

pub struct DeviceDatabase {
    root: Option<Tree>,
    order: usize,
    pub length: u64,
}

impl DeviceDatabase {
    pub fn new_empty(order: usize) -> DeviceDatabase {
        DeviceDatabase {
            root: None,
            length: 0,
            order: order,
        }
    }

    fn print_node(&self, node: &Tree, label: &str) {
        let devs: Vec<&u64> = node.devices.keys().collect();
        println!("{}: {:?}", label, devs);
    }

    pub fn add(&mut self, device: IoTDevice) {
        let mut node = if self.root.is_some() {
            mem::replace(&mut self.root, None).unwrap()
        } else {
            Node::new_leaf()
        };

        let (mut root, promoted) = self.add_r(node, device, true);

        if let Some(split_result) = promoted {
            let new_id = &split_result.0.clone().unwrap();
            root.add_key(new_id.numerical_id, split_result);
        }
        self.root = Some(root);
    }

    fn add_r(&mut self, node: Tree, device: IoTDevice, is_root: bool) -> (Tree, Option<Data>) {
        let mut node = node;
        let id = device.numerical_id;

        if node.is_leaf {
            if node.add_key(id, (Some(device), None)) {
                self.length += 1;
            }
        } else {
            // Remove the entry from  the device map to pass ownership into the recursion
            // always returns a result or the min_value of KeyType
            let (key, (dev, tree)) = node.remove_key(id).unwrap();
            let new = self.add_r(tree.unwrap(), device, false);

            // Re-add the key, device, and subtree to the map
            node.add_key(key, (dev, Some(new.0)));

            // In case a split happened in the add_r() call, the new device
            // "bubbles up" and needs to be added
            if let Some(split_result) = new.1 {
                let new_id = &split_result.0.clone().unwrap();
                node.add_key(new_id.numerical_id, split_result);
            }
        }

        if node.len() > self.order {
            let (new_parent, sibling) = node.split();

            // Check if the root node is "full" and add a new level
            if is_root {
                let mut parent = Node::new_regular();
                // Add the former root to the left
                parent.add_key(KeyType::min_value(), (None, Some(node)));
                // Add the new right part as well
                parent.add_key(new_parent.numerical_id, (Some(new_parent), Some(sibling)));
                (parent, None)
            } else {
                (node, Some((Some(new_parent), Some(sibling))))
            }
        } else {
            (node, None)
        }
    }

    pub fn is_a_valid_btree(&self) -> bool {
        if let Some(tree) = self.root.as_ref() {
            let total = self.validate(tree, 0);
            total.0 && total.1 == total.2
        } else {
            false // there is no tree
        }
    }

    fn validate(&self, node: &Tree, level: usize) -> (bool, usize, usize) {
        if node.is_leaf {
            (node.len() <= self.order, level, level)
        } else {
            // Root node only requires two children, every other node at least half the
            // order
            let min_children = if level > 0 { self.order / 2usize } else { 2 };
            let key_rules = node.len() <= self.order && node.len() >= min_children;

            let mut total = (key_rules, usize::max_value(), level);
            for n in node.devices.values() {
                if let Some(tree) = n.1.as_ref() {
                    let stats = self.validate(tree, level + 1);
                    total = (
                        total.0 && stats.0,
                        cmp::min(stats.1, total.1),
                        cmp::max(stats.2, total.2),
                    );
                }
            }
            total
        }
    }

    pub fn find(&self, id: KeyType) -> Option<IoTDevice> {
        match self.root.as_ref() {
            Some(tree) if id != KeyType::min_value() => self.find_r(tree, id),
            _ => None,
        }
    }

    fn find_r(&self, node: &Tree, id: KeyType) -> Option<IoTDevice> {
        match node.devices.get(&id) {
            Some(device) => device.0.clone(),
            None if !node.is_leaf => {
                let key = node.find_closest_key(id);
                if let Some(ref tree) = node.devices.get(&key) {
                    let tree = tree.1.as_ref().unwrap();
                    self.find_r(&tree, id)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /*   
    pub fn find(&self, numerical_id: KeyType) -> Option<IoTDevice> {
        self.find_r(&self.root, numerical_id)
    }
    
    fn find_r(&self, node: &Tree, numerical_id: u64) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                if n.dev.numerical_id == numerical_id {
                    Some(n.dev.clone())
                } else if n.dev.numerical_id < numerical_id {
                    self.find_r(&n.left, numerical_id)
                } else {
                    self.find_r(&n.right, numerical_id)
                }
            }
            _ => None,
        }
    }
    
    pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
        self.walk_in_order(&self.root, &callback);
    }
    
    fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice) -> ()) {
    
            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
    }*/
}
