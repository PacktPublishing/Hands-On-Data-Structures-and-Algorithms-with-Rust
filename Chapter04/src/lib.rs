#![feature(test)]

mod doubly_linked_list;
mod dynamic_array;
mod singly_linked_list;
mod skip_list;

#[cfg(test)]
mod tests {
    extern crate test;
    use crate::*;
    use rand::thread_rng;
    use rand::Rng;
    use std::collections::LinkedList;
    use test::Bencher;
    const LIST_ITEMS: u64 = 15_000;

    #[bench]
    fn bench_skip_list_find(b: &mut Bencher) {
        let mut list = skip_list::BestTransactionLog::new_empty(20);
        for i in 0..LIST_ITEMS {
            list.append(i, format!("INSERT INTO mytable VALUES ({})", i).to_owned());
        }
        let mut rng = thread_rng();

        b.iter(|| {
            list.find(rng.gen_range::<u64>(0, LIST_ITEMS))
                .expect("NOT FOUND")
        });
    }

    #[bench]
    fn bench_linked_list_find(b: &mut Bencher) {
        let mut list = doubly_linked_list::BetterTransactionLog::new_empty();
        let items: Vec<String> = (0..LIST_ITEMS).map(|i| format!("INSERT INTO mytable VALUES ({})", i).to_owned()).collect();
        for item in items.iter() {
            list.append(item.clone());
        }
        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.gen_range::<usize>(0, LIST_ITEMS as usize);
            list.iter().find(|x| x == &items[r]).expect("NOT FOUND")
        });
    }

    #[bench]
    fn bench_std_linked_list_find(b: &mut Bencher) {
        let mut list = std::collections::LinkedList::new();
        let items: Vec<String> = (0..LIST_ITEMS).map(|i| format!("INSERT INTO mytable VALUES ({})", i).to_owned()).collect();
        for item in items.iter() {
            list.push_back(item.clone());
        }
        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.gen_range::<usize>(0, LIST_ITEMS as usize);
            list.iter().find(|&x| x == &items[r]).expect("NOT FOUND")
        });
    }


    #[bench]
    fn bench_vec_find(b: &mut Bencher) {
        let mut list = vec![];

        for i in 0..LIST_ITEMS {
            list.push((i, format!("INSERT INTO mytable VALUES ({})", i).to_owned()));
        }
        let mut rng = thread_rng();

        b.iter(|| {
            let r = rng.gen_range::<u64>(0, LIST_ITEMS);
            list.iter().find(|&x| x.0 == r).expect("NOT FOUND")
        });
    }

    #[bench]
    fn bench_linked_list_append(b: &mut Bencher) {
        let mut list = LinkedList::new();
        let mut rng = thread_rng();

        b.iter(|| {
            list.push_back(rng.gen::<u64>())
        });
    }
    
    #[bench]
    fn bench_dynamic_array_append(b: &mut Bencher) {
        let mut list = dynamic_array::TimestampSaver::new_empty();
        let mut rng = thread_rng();

        b.iter(|| {
            list.append(rng.gen::<u64>())
        });
    }

      #[bench]
    fn bench_vec_append(b: &mut Bencher) {
        let mut list = vec![];
        let mut rng = thread_rng();

        b.iter(|| {
            list.push(rng.gen::<u64>())
        });
    }


    #[test]
    fn transaction_log_append() {
        let mut transaction_log = singly_linked_list::TransactionLog::new_empty();
        assert_eq!(transaction_log.length, 0);
        transaction_log.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        transaction_log.append("INSERT INTO mytable VALUES (2,3,4)".to_owned());
        transaction_log.append("INSERT INTO mytable VALUES (3,4,5)".to_owned());
        assert_eq!(transaction_log.length, 3);
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (2,3,4)".to_owned())
        );
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (3,4,5)".to_owned())
        );
        assert_eq!(transaction_log.pop(), None);
    }

    #[test]
    fn transaction_log_pop() {
        let mut list = singly_linked_list::TransactionLog::new_empty();
        assert_eq!(list.pop(), None);
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn better_transaction_log_append() {
        let mut transaction_log = doubly_linked_list::BetterTransactionLog::new_empty();
        assert_eq!(transaction_log.length, 0);
        transaction_log.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        transaction_log.append("INSERT INTO mytable VALUES (2,3,4)".to_owned());
        transaction_log.append("INSERT INTO mytable VALUES (3,4,5)".to_owned());
        assert_eq!(transaction_log.length, 3);
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (2,3,4)".to_owned())
        );
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (3,4,5)".to_owned())
        );
        assert_eq!(transaction_log.pop(), None);
    }

    #[test]
    fn better_transaction_log_pop() {
        let mut list = doubly_linked_list::BetterTransactionLog::new_empty();
        assert_eq!(list.pop(), None);
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn better_transaction_log_iterator() {
        let mut list = doubly_linked_list::BetterTransactionLog::new_empty();
        assert_eq!(list.pop(), None);
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (2,3,4)".to_owned());
        list.append("INSERT INTO mytable VALUES (3,4,5)".to_owned());
        let mut iter = list.clone().into_iter();
        assert_eq!(
            iter.next(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            iter.next(),
            Some("INSERT INTO mytable VALUES (2,3,4)".to_owned())
        );
        assert_eq!(
            iter.next(),
            Some("INSERT INTO mytable VALUES (3,4,5)".to_owned())
        );

        let mut iter = list.clone().back_iter();
        assert_eq!(
            iter.next_back(),
            Some("INSERT INTO mytable VALUES (3,4,5)".to_owned())
        );
        assert_eq!(
            iter.next_back(),
            Some("INSERT INTO mytable VALUES (2,3,4)".to_owned())
        );
        assert_eq!(
            iter.next_back(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
    }

    #[test]
    fn skip_list_append() {
        let mut list = skip_list::BestTransactionLog::new_empty(3);
        list.append(1, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append(2, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append(3, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append(4, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append(5, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append(6, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append(7, "INSERT INTO mytable VALUES (1,2,3)".to_owned());
        assert_eq!(list.length, 7);
    }

    #[test]
    fn skip_list_find() {
        let mut list = skip_list::BestTransactionLog::new_empty(3);
        list.append(1, "INSERT INTO mytable VALUES (1)".to_owned());
        list.append(2, "INSERT INTO mytable VALUES (2)".to_owned());
        list.append(3, "INSERT INTO mytable VALUES (3)".to_owned());
        list.append(4, "INSERT INTO mytable VALUES (4)".to_owned());
        list.append(5, "INSERT INTO mytable VALUES (5)".to_owned());
        list.append(6, "INSERT INTO mytable VALUES (6)".to_owned());
        list.append(7, "INSERT INTO mytable VALUES (7)".to_owned());
        assert_eq!(list.length, 7);
        assert_eq!(
            list.find(7),
            Some("INSERT INTO mytable VALUES (7)".to_owned())
        );
        assert_eq!(
            list.find(6),
            Some("INSERT INTO mytable VALUES (6)".to_owned())
        );
        assert_eq!(
            list.find(5),
            Some("INSERT INTO mytable VALUES (5)".to_owned())
        );
        assert_eq!(
            list.find(4),
            Some("INSERT INTO mytable VALUES (4)".to_owned())
        );
        assert_eq!(
            list.find(3),
            Some("INSERT INTO mytable VALUES (3)".to_owned())
        );
        assert_eq!(
            list.find(2),
            Some("INSERT INTO mytable VALUES (2)".to_owned())
        );
        assert_eq!(
            list.find(1),
            Some("INSERT INTO mytable VALUES (1)".to_owned())
        );
    }

    #[test]
    fn dynamic_array_append() {
        let mut list = dynamic_array::TimestampSaver::new_empty();
        let max: usize = 1_000;
        for i in 0..max {
            list.append(i as u64);
        }
        assert_eq!(list.length, max);
    }

    #[test]
    fn dynamic_array_at() {
        let mut list = dynamic_array::TimestampSaver::new_empty();
        let max: usize = 1_000;
        for i in 0..max {
            list.append(i as u64);
        }
        assert_eq!(list.length, max);
        for i in 0..max {
            assert_eq!(list.at(i), Some(i as u64));
        }
        assert_eq!(list.at(max + 1), None);
    }
     #[test]
    fn dynamic_array_iterate() {
        let mut list = dynamic_array::TimestampSaver::new_empty();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        assert_eq!(list.length, 4);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);

    }
}
