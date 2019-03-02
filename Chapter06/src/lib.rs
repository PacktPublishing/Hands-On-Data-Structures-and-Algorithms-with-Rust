#![feature(test, bind_by_move_pattern_guards)]

mod map;
mod set;

#[derive(Clone, Debug, PartialEq)]
pub struct LocationInformation {
    name: String,
    opened: String,
    address: String,
    security_group_name: String,
}
const MOD_ADLER: u32 = 65521;

pub fn adler32(bytes: &[u8]) -> u32 {
    let mut a = 1_u32;
    let mut b = 0_u32;

    for byte in bytes {
        a = (a + *byte as u32) % MOD_ADLER;
        b = (b + a) % MOD_ADLER;
    }

    (b << 16) | a
}

pub fn hashcode(bytes: &[u8]) -> u32 {
    let mut a = 0_u32;
    for (i, b) in bytes.iter().enumerate() {
        a ^= *b as u32;
        a <<= i % 4;
    }
    a
}

#[cfg(test)]
mod tests {
    extern crate test;
    use crate::*;
    use rand::{thread_rng, Rng};
    use std::cell::RefCell;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;
    use std::iter;
    use test::Bencher;

    const NO_ITEMS: usize = 10_000;
    thread_local!(static TEST_DATA: RefCell<Vec<(String, LocationInformation)>> = RefCell::new(random_location_info(NO_ITEMS)));

    fn new_location_info(name: &str) -> LocationInformation {
        LocationInformation {
            name: name.to_owned(),
            opened: "2018".to_owned(),
            address: "1 Curved Way, 10223 Someplace".to_owned(),
            security_group_name: format!("{}-sec", name),
        }
    }

    fn random_location_info(n: usize) -> Vec<(String, LocationInformation)> {
        let s = (b'A'..b'Z').map(|a| a as char).collect::<Vec<char>>();
        let rng = RefCell::new(thread_rng());

        iter::repeat(s)
            .take(n)
            .map(|mut s| {
                rng.borrow_mut().shuffle(&mut s);
                let length = rng.borrow_mut().gen_range(10, s.len());
                let k = s.into_iter().take(length).collect::<String>();
                let val = new_location_info(&k);
                (k, val)
            })
            .collect::<Vec<(String, LocationInformation)>>()
    }

    #[bench]
    fn bench_hash_map_hashcode(b: &mut Bencher) {
        TEST_DATA.with(|location_info_cell| {
            let n = NO_ITEMS;
            let mut m =
                map::LocationCache::new(Box::new(|e: &String| hashcode(e.as_bytes()) as usize), n);
            let location_info = location_info_cell.borrow();
            for (key, value) in location_info.iter() {
                m.insert(key.clone(), value.clone());
            }

            assert_eq!(m.length, location_info.len());
            let mut rng = thread_rng();
            b.iter(|| {
                let pair = rng.choose(&location_info).expect("Nothing to choose from");
                m.get(&pair.0).expect("Not Found");
            });
        });
    }

    #[bench]
    fn bench_hash_map_adler32(b: &mut Bencher) {
        TEST_DATA.with(|location_info_cell| {
            let n = NO_ITEMS;
            let mut m =
                map::LocationCache::new(Box::new(|e: &String| adler32(e.as_bytes()) as usize), n);
            let location_info = location_info_cell.borrow();
            for (key, value) in location_info.iter() {
                m.insert(key.clone(), value.clone());
            }

            assert_eq!(m.length, location_info.len());
            let mut rng = thread_rng();
            b.iter(|| {
                let pair = rng.choose(&location_info).expect("Nothing to choose from");
                m.get(&pair.0).expect("Not Found");
            });
        });
    }

    #[bench]
    fn bench_hash_map_defaulthasher(b: &mut Bencher) {
        TEST_DATA.with(|location_info_cell| {
            let n = NO_ITEMS;
            let mut m = map::LocationCache::new(
                Box::new(|e: &String| {
                    let mut h = DefaultHasher::new();
                    e.hash(&mut h);
                    h.finish() as usize
                }),
                n,
            );
            let location_info = location_info_cell.borrow();
            for (key, value) in location_info.iter() {
                m.insert(key.clone(), value.clone());
            }

            assert_eq!(m.length, location_info.len());
            let mut rng = thread_rng();
            b.iter(|| {
                let pair = rng.choose(&location_info).expect("Nothing to choose from");
                m.get(&pair.0).expect("Not Found");
            });
        });
    }

    #[bench]
    fn bench_std_hash_map(b: &mut Bencher) {
        TEST_DATA.with(|location_info_cell| {
            let n = NO_ITEMS;
            let mut m = std::collections::HashMap::with_capacity(n);
            let location_info = location_info_cell.borrow();
            for (key, value) in location_info.iter() {
                m.insert(key.clone(), value.clone());
            }

            assert_eq!(m.len(), location_info.len());
            let mut rng = thread_rng();
            b.iter(|| {
                let pair = rng.choose(&location_info).expect("Nothing to choose from");
                m.get(&pair.0).expect("Not Found");
            });
        });
    }

    #[test]
    fn hashcode_test() {
        let payload = "hello".to_string();
        assert_eq!(hashcode(payload.as_bytes()), 3535);
        assert_eq!(hashcode("".to_owned().as_bytes()), 0);
        assert_eq!(hashcode("ÄZZZZZZZZZZZ".to_owned().as_bytes()), 25652682);
    }

    #[test]
    fn adler32_test() {
        let payload = "hello".to_owned().into_bytes();
        assert_eq!(adler32(&payload), 103547413);
        assert_eq!(adler32(&"".to_owned().into_bytes()), 1);
        assert_eq!(adler32(&"ÄZZZZZZZZZZZ".to_owned().into_bytes()), 660079910);
    }

    #[test]
    fn trie_set_insert() {
        let mut m = set::NetworkDeviceStore::new_empty();
        let devices = vec![
            vec![172, 0, 0, 1],
            vec![192, 168, 2, 1],
            vec![172, 11, 0, 1],
            vec![1, 1, 1, 1],
        ];

        for d in devices.iter() {
            m.insert(d)
        }

        assert_eq!(m.length, devices.len() as u64);
    }

    #[test]
    fn trie_set_contains() {
        let mut m = set::NetworkDeviceStore::new_empty();
        let devices = vec![
            vec![172, 0, 0, 1],
            vec![192, 168, 2, 1],
            vec![172, 11, 0, 1],
            vec![1, 1, 1, 1],
        ];

        for d in devices.iter() {
            m.insert(d);
        }

        assert_eq!(m.length, devices.len() as u64);

        for d in devices.iter() {
            assert!(m.contains(d));
        }
        assert_eq!(m.contains(&vec![8, 8, 8, 8]), false);
    }
    #[test]
    fn trie_set_intersection() {
        let mut a = set::NetworkDeviceStore::new_empty();
        let mut b = set::NetworkDeviceStore::new_empty();

        let devices_a = vec![vec![172, 0, 0, 1], vec![1, 1, 1, 1], vec![192, 168, 2, 1]];
        let devices_b = vec![vec![172, 11, 0, 1], vec![1, 1, 1, 1]];

        for d in devices_a.iter() {
            a.insert(d);
        }

        assert_eq!(a.length, devices_a.len() as u64);

        for d in devices_b.iter() {
            b.insert(d)
        }
        assert_eq!(b.length, devices_b.len() as u64);

        let a_and_b = a.intersection(b);
        assert_eq!(a_and_b.length, 1);
        let set: Vec<Vec<u8>> = a_and_b.into_iter().collect();
        assert_eq!(set, vec![vec![1, 1, 1, 1]]);
    }

    #[test]
    fn trie_set_difference() {
        let mut a = set::NetworkDeviceStore::new_empty();
        let mut b = set::NetworkDeviceStore::new_empty();

        let devices_a = vec![vec![172, 0, 0, 1], vec![192, 168, 2, 1], vec![1, 1, 1, 1]];
        let devices_b = vec![vec![172, 11, 0, 1], vec![1, 1, 1, 1]];

        for d in devices_a.iter() {
            a.insert(d);
        }

        assert_eq!(a.length, devices_a.len() as u64);

        for d in devices_b.iter() {
            b.insert(d);
        }
        assert_eq!(b.length, devices_b.len() as u64);

        let a_not_b = a.difference(b);
        assert_eq!(a_not_b.length, (devices_a.len()) as u64 - 1);

        for d in devices_a.iter().take(2) {
            assert!(a_not_b.contains(d));
        }
    }

    #[test]
    fn trie_set_union() {
        let mut a = set::NetworkDeviceStore::new_empty();
        let mut b = set::NetworkDeviceStore::new_empty();

        let devices_a = vec![vec![172, 0, 0, 1], vec![192, 168, 2, 1], vec![1, 1, 1, 1]];
        let devices_b = vec![vec![172, 11, 0, 1], vec![1, 1, 1, 1]];

        for d in devices_a.iter() {
            a.insert(d)
        }

        assert_eq!(a.length, devices_a.len() as u64);

        for d in devices_b.iter() {
            b.insert(d)
        }
        assert_eq!(b.length, devices_b.len() as u64);

        let a_or_b = a.union(b);
        assert_eq!(
            a_or_b.length,
            (devices_b.len() + devices_a.len()) as u64 - 1
        );

        for d in devices_b.iter().chain(devices_a.iter()) {
            assert!(a_or_b.contains(d));
        }
    }

    #[test]
    fn hash_map_insert() {
        let mut m = map::LocationCache::new(Box::new(|e: &String| e.len()), 3);
        let val = vec![
            (
                "BlackRockSpire".to_owned(),
                new_location_info("BlackRockSpire"),
            ),
            ("MoltenCore".to_owned(), new_location_info("MoltenCore")),
            (
                "RagefireChasm".to_owned(),
                new_location_info("RagefireChasm"),
            ),
        ];

        m.insert(val[0].0.clone(), val[0].1.clone());
        m.insert(val[1].0.clone(), val[1].1.clone());
        m.insert(val[2].0.clone(), val[2].1.clone());

        assert_eq!(m.length, val.len());
    }

    #[test]
    fn hash_map_remove() {
        let mut m = map::LocationCache::new(Box::new(|e: &String| e.len()), 3);
        let val = vec![
            (
                "BlackRockSpire".to_owned(),
                new_location_info("BlackRockSpire"),
            ),
            ("MoltenCore".to_owned(), new_location_info("MoltenCore")),
            (
                "RagefireChasm".to_owned(),
                new_location_info("RagefireChasm"),
            ),
        ];

        m.insert(val[0].0.clone(), val[0].1.clone());
        m.insert(val[1].0.clone(), val[1].1.clone());
        m.insert(val[2].0.clone(), val[2].1.clone());

        assert_eq!(m.length, val.len());

        assert_eq!(m.remove(val[0].0.clone()), Some(val[0].1.clone()));
        assert_eq!(m.remove(val[1].0.clone()), Some(val[1].1.clone()));
        assert_eq!(m.length, 1);
    }

    #[test]
    fn hash_map_get() {
        let mut m = map::LocationCache::new(Box::new(|e: &String| e.len()), 3);
        let val = vec![
            (
                "BlackRockSpire".to_owned(),
                new_location_info("BlackRockSpire"),
            ),
            ("MoltenCore".to_owned(), new_location_info("MoltenCore")),
            (
                "RagefireChasm".to_owned(),
                new_location_info("RagefireChasm"),
            ),
        ];

        m.insert(val[0].0.clone(), val[0].1.clone());
        m.insert(val[1].0.clone(), val[1].1.clone());
        m.insert(val[2].0.clone(), val[2].1.clone());

        assert_eq!(m.length, val.len());

        assert_eq!(m.get(&val[0].0), Some(val[0].1.clone()));
        assert_eq!(m.get(&val[1].0), Some(val[1].1.clone()));
        assert_eq!(m.length, val.len());
    }
}
