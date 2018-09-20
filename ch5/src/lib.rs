#![feature(uniform_paths, test)]

mod binary_search_tree;
mod red_black_tree;
mod heap;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String,
}

impl IoTDevice {
    pub fn new(id: u64, address: impl Into<String>) -> IoTDevice {
        IoTDevice {
            address: address.into(),
            numerical_id: id,
        }
    }
}

impl PartialEq for IoTDevice {
    fn eq(&self, other: &IoTDevice) -> bool {
        self.numerical_id == other.numerical_id && self.address == other.address
    }
}

#[derive(Clone, Debug)]
pub struct MessageNotification {
    pub no_messages: u64,
    pub device: IoTDevice,
}

impl MessageNotification {
    pub fn new(device: IoTDevice, no_messages: u64) -> MessageNotification {
        MessageNotification {
            no_messages: no_messages,
            device: device,
        }
    }
}


impl PartialEq for MessageNotification {
    fn eq(&self, other: &MessageNotification) -> bool {
        self.device.eq(&other.device) && self.no_messages == other.no_messages
    }
}


#[cfg(test)]
mod tests {
    extern crate test;
    use crate::*;
    use rand::thread_rng;
    use rand::Rng;
    use std::cell::RefCell;
    use test::Bencher;

    const LIST_ITEMS: u64 = 10_000;

    fn new_device_with_id(id: u64) -> IoTDevice {
        IoTDevice::new(id, format!("My address is {}", id))
    }


    fn new_notification_with_id(id: u64, no_messages: u64) -> MessageNotification {
        let dev = IoTDevice::new(id, format!("My address is {}", id));
        MessageNotification::new(dev, no_messages)
    }


    #[bench]
    fn bench_unsorted_insert_bst_find(b: &mut Bencher) {
        let mut tree = binary_search_tree::DeviceRegistry::new_empty();
        let mut items: Vec<IoTDevice> = (0..LIST_ITEMS).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items {
            tree.add(item);
        }

        assert_eq!(tree.length, LIST_ITEMS);

        b.iter(|| {
            let r = rng.gen_range::<u64>(0, LIST_ITEMS);
            tree.find(r).expect("NOT FOUND")
        });
    }

    #[bench]
    fn bench_sorted_insert_bst_find(b: &mut Bencher) {
        let mut tree = binary_search_tree::DeviceRegistry::new_empty();

        for i in 0..LIST_ITEMS {
            tree.add(new_device_with_id(i));
        }
        assert_eq!(tree.length, LIST_ITEMS);
        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.gen_range::<u64>(0, LIST_ITEMS);
            tree.find(r).expect("NOT FOUND")
        });
    }

    #[test]
    fn binary_search_tree_add() {
        let mut tree = binary_search_tree::DeviceRegistry::new_empty();
        tree.add(new_device_with_id(4));
        tree.add(new_device_with_id(3));
        tree.add(new_device_with_id(2));
        tree.add(new_device_with_id(1));
        tree.add(new_device_with_id(5));
        tree.add(new_device_with_id(6));
        tree.add(new_device_with_id(7));
        assert_eq!(tree.length, 7);
    }

    #[test]
    fn binary_search_tree_walk_in_order() {
        let len = 10;

        let mut tree = binary_search_tree::DeviceRegistry::new_empty();
        let mut items: Vec<IoTDevice> = (0..len).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items.iter() {
            tree.add(item.clone());
        }

        assert_eq!(tree.length, len);
        let v: RefCell<Vec<IoTDevice>> = RefCell::new(vec![]);
        tree.walk(|n| v.borrow_mut().push(n.clone()));
        let mut items = items;
        // sort in descending order:
        items.sort_by(|a, b| b.numerical_id.cmp(&a.numerical_id));
        assert_eq!(v.into_inner(), items)
    }

    #[test]
    fn binary_search_tree_find() {
        let mut tree = binary_search_tree::DeviceRegistry::new_empty();

        tree.add(new_device_with_id(4));
        tree.add(new_device_with_id(3));
        tree.add(new_device_with_id(2));
        tree.add(new_device_with_id(1));
        tree.add(new_device_with_id(5));
        tree.add(new_device_with_id(6));
        tree.add(new_device_with_id(7));
        assert_eq!(tree.find(100), None);
        assert_eq!(tree.find(4), Some(new_device_with_id(4)));
        assert_eq!(tree.find(3), Some(new_device_with_id(3)));
        assert_eq!(tree.find(2), Some(new_device_with_id(2)));
        assert_eq!(tree.find(1), Some(new_device_with_id(1)));
        assert_eq!(tree.find(5), Some(new_device_with_id(5)));
        assert_eq!(tree.find(6), Some(new_device_with_id(6)));
        assert_eq!(tree.find(7), Some(new_device_with_id(7)));
        assert_eq!(tree.length, 7);
    }

    #[bench]
    fn bench_unsorted_insert_rbt_find(b: &mut Bencher) {
        let mut tree = red_black_tree::BetterDeviceRegistry::new_empty();
        let mut items: Vec<IoTDevice> = (0..LIST_ITEMS).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items {
            tree.add(item);
        }

        assert_eq!(tree.length, LIST_ITEMS);

        b.iter(|| {
            let r = rng.gen_range::<u64>(0, LIST_ITEMS);
            tree.find(r).expect("NOT FOUND")
        });
    }

    #[bench]
    fn bench_sorted_insert_rbt_find(b: &mut Bencher) {
        let mut tree = red_black_tree::BetterDeviceRegistry::new_empty();

        for i in 0..LIST_ITEMS {
            tree.add(new_device_with_id(i));
        }
        assert_eq!(tree.length, LIST_ITEMS);
        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.gen_range::<u64>(0, LIST_ITEMS);
            tree.find(r).expect("NOT FOUND")
        });
    }

    #[test]
    fn red_black_tree_add() {
        let mut tree = red_black_tree::BetterDeviceRegistry::new_empty();
        tree.add(new_device_with_id(1));
        tree.add(new_device_with_id(2));
        tree.add(new_device_with_id(3));
        tree.add(new_device_with_id(4));
        tree.add(new_device_with_id(5));
        tree.add(new_device_with_id(6));
        tree.add(new_device_with_id(7));
        assert_eq!(tree.length, 7);
        assert!(tree.is_a_valid_red_black_tree());
    }

    #[test]
    fn red_black_tree_walk_in_order() {
        let len = 10;

        let mut tree = red_black_tree::BetterDeviceRegistry::new_empty();
        let mut items: Vec<IoTDevice> = (0..len).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items.iter() {
            tree.add(item.clone());
        }
        assert!(tree.is_a_valid_red_black_tree());
        assert_eq!(tree.length, len);
        let v: RefCell<Vec<IoTDevice>> = RefCell::new(vec![]);
        tree.walk(|n| v.borrow_mut().push(n.clone()));
        let mut items = items;
        // sort in descending order:
        items.sort_by(|a, b| b.numerical_id.cmp(&a.numerical_id));
        assert_eq!(v.into_inner(), items)
    }

    #[test]
    fn red_black_tree_find() {
        let mut tree = red_black_tree::BetterDeviceRegistry::new_empty();

        tree.add(new_device_with_id(3));
        tree.add(new_device_with_id(2));
        tree.add(new_device_with_id(1));
        tree.add(new_device_with_id(6));
        tree.add(new_device_with_id(4));
        tree.add(new_device_with_id(5));
        tree.add(new_device_with_id(7));

        assert!(tree.is_a_valid_red_black_tree());
        assert_eq!(tree.length, 7);

        assert_eq!(tree.find(100), None);
        assert_eq!(tree.find(4), Some(new_device_with_id(4)));
        assert_eq!(tree.find(3), Some(new_device_with_id(3)));
        assert_eq!(tree.find(2), Some(new_device_with_id(2)));
        assert_eq!(tree.find(1), Some(new_device_with_id(1)));
        assert_eq!(tree.find(5), Some(new_device_with_id(5)));
        assert_eq!(tree.find(6), Some(new_device_with_id(6)));
        assert_eq!(tree.find(7), Some(new_device_with_id(7)));
    }


    #[test]
    fn binary_heap_add() {
        let mut heap = heap::MessageChecker::new_empty();  

        heap.add(new_notification_with_id(1, 100));
        heap.add(new_notification_with_id(2, 200));
        heap.add(new_notification_with_id(3, 500));
        heap.add(new_notification_with_id(4, 40));
        assert_eq!(heap.length, 4);

    }


    #[test]
    fn binary_heap_pop() {
        let mut heap = heap::MessageChecker::new_empty();  

        let a = new_notification_with_id(1, 40);
        let b = new_notification_with_id(2, 300);
        let c = new_notification_with_id(3, 50);
        let d = new_notification_with_id(4, 500);

        heap.add(a.clone());
        heap.add(b.clone());
        heap.add(c.clone());
        heap.add(d.clone());

        assert_eq!(heap.length, 4);

        assert_eq!(heap.pop(), Some(d));
        assert_eq!(heap.pop(), Some(b));
        assert_eq!(heap.pop(), Some(c));
        assert_eq!(heap.pop(), Some(a));
    }
}
