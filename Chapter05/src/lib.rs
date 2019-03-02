#![feature(test, bind_by_move_pattern_guards)]

mod binary_search_tree;
mod btree;
mod graph;
mod heap;
mod red_black_tree;
mod trie;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub path: String,
    pub address: String,
}

impl IoTDevice {
    pub fn new(id: u64, address: impl Into<String>, path: impl Into<String>) -> IoTDevice {
        IoTDevice {
            address: address.into(),
            numerical_id: id,
            path: path.into(),
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
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use test::Bencher;

    const LIST_ITEMS: u64 = 1_000;

    fn new_device_with_id(id: u64) -> IoTDevice {
        new_device_with_id_path(id, "")
    }

    fn new_device_with_id_path(id: u64, path: impl Into<String>) -> IoTDevice {
        IoTDevice::new(id, format!("My address is {}", id), path)
    }

    fn new_notification_with_id(id: u64, no_messages: u64) -> MessageNotification {
        let dev = new_device_with_id(id);
        MessageNotification::new(dev, no_messages)
    }

    fn build_graph(g: graph::InternetOfThings, items: &Vec<IoTDevice>) -> graph::InternetOfThings {
        let mut g = g;

        g.set_nodes(items.iter().map(|n| n.numerical_id.clone()).collect());
        g.set_edges(
            items[0].numerical_id.clone(),
            vec![
                (1, items[1].numerical_id.clone()),
                (1, items[2].numerical_id.clone()),
                (1, items[3].numerical_id.clone()),
                (10, items[9].numerical_id.clone()),
            ],
        );

        g.set_edges(
            items[1].numerical_id.clone(),
            vec![(1, items[0].numerical_id.clone())],
        );
        g.set_edges(
            items[2].numerical_id.clone(),
            vec![(1, items[0].numerical_id.clone())],
        );
        g.set_edges(
            items[3].numerical_id.clone(),
            vec![
                (1, items[0].numerical_id.clone()),
                (1, items[4].numerical_id.clone()),
            ],
        );
        g.set_edges(
            items[4].numerical_id.clone(),
            vec![
                (1, items[3].numerical_id.clone()),
                (1, items[5].numerical_id.clone()),
            ],
        );
        g.set_edges(
            items[5].numerical_id.clone(),
            vec![
                (1, items[4].numerical_id.clone()),
                (1, items[6].numerical_id.clone()),
            ],
        );
        g.set_edges(
            items[6].numerical_id.clone(),
            vec![
                (1, items[9].numerical_id.clone()),
                (1, items[5].numerical_id.clone()),
            ],
        );
        g.set_edges(
            items[7].numerical_id.clone(),
            vec![(1, items[9].numerical_id.clone())],
        );
        g.set_edges(
            items[8].numerical_id.clone(),
            vec![(1, items[9].numerical_id.clone())],
        );
        g.set_edges(
            items[9].numerical_id.clone(),
            vec![
                (1, items[8].numerical_id.clone()),
                (1, items[7].numerical_id.clone()),
                (1, items[6].numerical_id.clone()),
                (10, items[0].numerical_id.clone()),
            ],
        );
        g
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

        let items: Vec<IoTDevice> = (0..LIST_ITEMS).map(new_device_with_id).collect();

        for item in items {
            tree.add(item);
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
    fn trie_add() {
        let mut trie = trie::BestDeviceRegistry::new_empty();
        let len = 10;

        let mut rng = thread_rng();

        for i in 0..len {
            trie.add(new_device_with_id_path(
                i,
                format!("factory{}/machineA/{}", rng.gen_range(0, len), i),
            ));
        }

        assert_eq!(trie.length, len);
    }

    #[test]
    fn trie_walk_in_order() {
        let mut trie = trie::BestDeviceRegistry::new_empty();
        let len = 10;

        let mut rng = thread_rng();
        let items: Vec<IoTDevice> = (0..len)
            .map(|i| {
                new_device_with_id_path(
                    i,
                    format!("factory{}/machineA/{}", rng.gen_range(0, len), i),
                )
            })
            .collect();

        for item in items.iter() {
            trie.add(item.clone());
        }
        assert_eq!(trie.length, len);
        let v: RefCell<Vec<IoTDevice>> = RefCell::new(vec![]);
        trie.walk(|n| v.borrow_mut().push(n.clone()));
        let mut items = items;
        // sort in descending order:
        items.sort_by(|a, b| b.numerical_id.cmp(&a.numerical_id));
        let mut actual = v.into_inner();
        actual.sort_by(|a, b| b.numerical_id.cmp(&a.numerical_id));
        assert_eq!(actual, items)
    }

    #[test]
    fn trie_find() {
        let mut trie = trie::BestDeviceRegistry::new_empty();
        let len = 10;

        let mut rng = thread_rng();
        let mut paths = vec![];
        for i in 0..len {
            let s = format!("factory{}/machineA/{}", rng.gen_range(0, len), i);
            trie.add(new_device_with_id_path(i, s.clone()));
            paths.push(s);
        }

        assert_eq!(trie.length, len);
        assert_eq!(trie.find("100"), None);
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

    #[bench]
    fn bench_unsorted_insert_btree_find_7(b: &mut Bencher) {
        let mut tree = btree::DeviceDatabase::new_empty(7);
        let mut items: Vec<IoTDevice> = (1..=LIST_ITEMS).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items {
            tree.add(item);
        }
        assert_eq!(tree.length, LIST_ITEMS);
        assert!(tree.is_a_valid_btree());
        b.iter(|| {
            let r = rng.gen_range::<u64>(1, LIST_ITEMS + 1);
            tree.find(r).expect("NOT FOUND")
        });
    }

  #[bench]
    fn bench_unsorted_insert_btree_find_14(b: &mut Bencher) {
        let mut tree = btree::DeviceDatabase::new_empty(14);
        let mut items: Vec<IoTDevice> = (1..=LIST_ITEMS).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items {
            tree.add(item);
        }
        assert_eq!(tree.length, LIST_ITEMS);
        assert!(tree.is_a_valid_btree());
        b.iter(|| {
            let r = rng.gen_range::<u64>(1, LIST_ITEMS + 1);
            tree.find(r).expect("NOT FOUND")
        });
    }


    #[bench]
    fn bench_unsorted_insert_btree_find_6(b: &mut Bencher) {
        let mut tree = btree::DeviceDatabase::new_empty(6);
        let mut items: Vec<IoTDevice> = (1..=LIST_ITEMS).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items {
            tree.add(item);
        }
        assert_eq!(tree.length, LIST_ITEMS);
        assert!(tree.is_a_valid_btree());
        b.iter(|| {
            let r = rng.gen_range::<u64>(1, LIST_ITEMS + 1);
            tree.find(r).expect("NOT FOUND")
        });
    }

    #[bench]
    fn bench_unsorted_insert_btree_find_5(b: &mut Bencher) {
        let mut tree = btree::DeviceDatabase::new_empty(5);
        let mut items: Vec<IoTDevice> = (1..=LIST_ITEMS).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items {
            tree.add(item);
        }
        assert_eq!(tree.length, LIST_ITEMS);
        assert!(tree.is_a_valid_btree());
        b.iter(|| {
            let r = rng.gen_range::<u64>(1, LIST_ITEMS + 1);
            tree.find(r).expect("NOT FOUND")
        });
    }

    #[bench]
    fn bench_unsorted_insert_btree_find_4(b: &mut Bencher) {
        let mut tree = btree::DeviceDatabase::new_empty(4);
        let mut items: Vec<IoTDevice> = (1..=LIST_ITEMS).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items {
            tree.add(item);
        }
        assert_eq!(tree.length, LIST_ITEMS);
        assert!(tree.is_a_valid_btree());
        b.iter(|| {
            let r = rng.gen_range::<u64>(1, LIST_ITEMS + 1);
            tree.find(r).expect("NOT FOUND")
        });
    }

    #[bench]
    fn bench_unsorted_insert_btree_find_3(b: &mut Bencher) {
        let mut tree = btree::DeviceDatabase::new_empty(3);
        let mut items: Vec<IoTDevice> = (1..=LIST_ITEMS).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items {
            tree.add(item);
        }
        assert_eq!(tree.length, LIST_ITEMS);
        assert!(tree.is_a_valid_btree());
        b.iter(|| {
            let r = rng.gen_range::<u64>(1, LIST_ITEMS + 1);
            tree.find(r).expect("NOT FOUND")
        });
    }

    #[bench]
    fn bench_sorted_insert_btree_find_4(b: &mut Bencher) {
        let mut tree = btree::DeviceDatabase::new_empty(4);

        let items: Vec<IoTDevice> = (1..=LIST_ITEMS).map(new_device_with_id).collect();

        for item in items {
            tree.add(item);
        }

        assert_eq!(tree.length, LIST_ITEMS);
        assert!(tree.is_a_valid_btree());

        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.gen_range::<u64>(1, LIST_ITEMS + 1);
            tree.find(r).expect("NOT FOUND")
        });
    }


    #[bench]
    fn bench_sorted_insert_btreemap_find(b: &mut Bencher) {
        let mut tree = std::collections::BTreeMap::new();

        let items: Vec<IoTDevice> = (1..=LIST_ITEMS).map(new_device_with_id).collect();

        for item in items {
            tree.insert(item.numerical_id, item);
        }

        assert_eq!(tree.len(), LIST_ITEMS as usize);

        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.gen_range::<u64>(1, LIST_ITEMS + 1);
            tree.get(&r).expect("NOT FOUND")
        });
    }

    #[test]
    fn btree_add() {
        let mut tree = btree::DeviceDatabase::new_empty(3);
        tree.add(new_device_with_id(0));
        tree.add(new_device_with_id(2));
        tree.add(new_device_with_id(4));
        tree.add(new_device_with_id(3));
        tree.add(new_device_with_id(5));
        tree.add(new_device_with_id(6));
        tree.add(new_device_with_id(7));

        assert_eq!(tree.length, 7);
        assert!(tree.is_a_valid_btree());
    }

    #[test]
    fn btree_walk_in_order() {
        let len = 7;

        let mut tree = btree::DeviceDatabase::new_empty(3);
        let mut items: Vec<IoTDevice> = (0..len).map(new_device_with_id).collect();

        let mut rng = thread_rng();
        rng.shuffle(&mut items);

        for item in items.iter() {
            tree.add(item.clone());
        }
        assert!(tree.is_a_valid_btree());
        assert_eq!(tree.length, len);
        let v: RefCell<Vec<IoTDevice>> = RefCell::new(vec![]);
        tree.walk(|n| v.borrow_mut().push(n.clone()));
        let mut items = items;
        // sort in descending order:
        items.sort_by(|a, b| a.numerical_id.cmp(&b.numerical_id));
        assert_eq!(v.into_inner(), items)
    }

    #[test]
    fn btree_find() {
        let mut tree = btree::DeviceDatabase::new_empty(3);

        tree.add(new_device_with_id(3));
        tree.add(new_device_with_id(2));
        tree.add(new_device_with_id(1));
        tree.add(new_device_with_id(6));
        tree.add(new_device_with_id(4));
        tree.add(new_device_with_id(5));
        tree.add(new_device_with_id(7));

        assert!(tree.is_a_valid_btree());
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
    fn graph_insert_edges() {
        let len = 10;
        let items: Vec<IoTDevice> = (0..len).map(new_device_with_id).collect();

        let g = build_graph(graph::InternetOfThings::new(), &items);

        assert_eq!(g.edges(), 20);
        assert_eq!(g.nodes(), len as usize);
    }

    #[test]
    fn graph_find_shortest_path() {
        let len = 10;
        let items: Vec<IoTDevice> = (0..len).map(new_device_with_id).collect();

        let g = build_graph(graph::InternetOfThings::new(), &items);

        assert_eq!(g.edges(), 20);
        assert_eq!(g.nodes(), len as usize);

        assert_eq!(
            g.shortest_path(items[0].numerical_id, items[9].numerical_id),
            Some((
                5,
                vec![
                    items[0].numerical_id,
                    items[3].numerical_id,
                    items[4].numerical_id,
                    items[5].numerical_id,
                    items[6].numerical_id,
                    items[9].numerical_id
                ]
            ))
        )
    }

    #[test]
    fn graph_neighbors() {
        let len = 10;
        let items: Vec<IoTDevice> = (0..len).map(new_device_with_id).collect();

        let g = build_graph(graph::InternetOfThings::new(), &items);

        assert_eq!(g.edges(), 20);
        assert_eq!(g.nodes(), len as usize);

        assert_eq!(
            g.connected(items[0].numerical_id, 1),
            Some(HashSet::from_iter(
                vec![
                    items[1].numerical_id,
                    items[2].numerical_id,
                    items[3].numerical_id,
                    items[9].numerical_id,
                ]
                .into_iter()
            ))
        )
    }
}
