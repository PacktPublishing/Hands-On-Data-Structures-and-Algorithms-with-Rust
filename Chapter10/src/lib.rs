#![feature(test, bind_by_move_pattern_guards)]
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::mem;

pub fn linear_search<T: Eq + Clone>(haystack: &[T], needle: &T) -> Option<usize> {
    for (i, h) in haystack.iter().enumerate() {
        if h.eq(needle) {
            return Some(i);
        }
    }
    None
}

pub fn jump_search<T: Eq + PartialOrd + Clone + Debug>(
    haystack: &[T],
    needle: &T,
    jump_size: usize,
) -> Option<usize> {
    if jump_size < haystack.len() {
        let mut i = 0;
        while i < haystack.len() - 1 {
            if i + jump_size < haystack.len() {
                i += jump_size
            } else {
                i = haystack.len() - 1;
            }
            if &haystack[i] == needle {
                return Some(i);
            } else if &haystack[i] > needle {
                return linear_search(&haystack[(i - jump_size)..i], needle);
            }
        }
    }
    None
}

pub fn binary_search<T: Eq + PartialOrd>(haystack: &[T], needle: &T) -> Option<usize> {
    let (mut left, mut right) = (0, haystack.len() - 1);
    while left <= right {
        let pivot = left + (right - left) / 2;
        if needle < &haystack[pivot] {
            right = pivot - 1;
        } else if needle > &haystack[pivot] {
            left = pivot + 1;
        } else {
            return Some(pivot); // lucky find
        }
    }
    None
}

#[cfg(test)]
mod tests {
    extern crate test;
    use crate::*;
    use rand::{thread_rng, Rng};
    use test::Bencher;

    fn random_number_vec(size: usize) -> Vec<i64> {
        let mut v: Vec<i64> = (0..size as i64).collect();
        let mut rng = thread_rng();
        rng.shuffle(&mut v);
        v
    }

    #[bench]
    fn bench_linear_search_1k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).collect();
        let mut rng = thread_rng();
        b.iter(|| assert!(linear_search(&items, &rng.choose(&items).unwrap()).is_some()));
    }

    #[bench]
    fn bench_linear_search_5k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).collect();
        let mut rng = thread_rng();
        b.iter(|| assert!(linear_search(&items, &rng.choose(&items).unwrap()).is_some()));
    }

    #[bench]
    fn bench_linear_search_10k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).collect();
        let mut rng = thread_rng();
        b.iter(|| assert!(linear_search(&items, &rng.choose(&items).unwrap()).is_some()));
    }

    #[bench]
    fn bench_jump_search_1k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).collect();
        let mut rng = thread_rng();
        let jump_size: usize = (items.len() as f64 * 0.3).floor() as usize;
        b.iter(|| assert!(jump_search(&items, &rng.choose(&items).unwrap(), jump_size).is_some()));
    }

    #[bench]
    fn bench_jump_search_5k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).collect();
        let mut rng = thread_rng();
        let jump_size: usize = (items.len() as f64 * 0.3).floor() as usize;
        b.iter(|| assert!(jump_search(&items, &rng.choose(&items).unwrap(), jump_size).is_some()));
    }

    #[bench]
    fn bench_jump_search_10k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).collect();
        let mut rng = thread_rng();
        let jump_size: usize = (items.len() as f64 * 0.3).floor() as usize;
        b.iter(|| assert!(jump_search(&items, &rng.choose(&items).unwrap(), jump_size).is_some()));
    }

    #[bench]
    fn bench_binary_search_1k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).collect();
        let mut rng = thread_rng();
        b.iter(|| assert!(binary_search(&items, &rng.choose(&items).unwrap()).is_some()));
    }

    #[bench]
    fn bench_binary_search_5k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).collect();
        let mut rng = thread_rng();
        b.iter(|| assert!(binary_search(&items, &rng.choose(&items).unwrap()).is_some()));
    }

    #[bench]
    fn bench_binary_search_10k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).collect();
        let mut rng = thread_rng();
        b.iter(|| assert!(binary_search(&items, &rng.choose(&items).unwrap()).is_some()));
    }

    #[bench]
    fn bench_std_binary_search_1k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).collect();
        let mut rng = thread_rng();
        b.iter(|| assert!(&items.binary_search(&rng.choose(&items).unwrap()).is_ok()));
    }

    #[bench]
    fn bench_std_binary_search_5k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).collect();
        let mut rng = thread_rng();
        b.iter(|| assert!(&items.binary_search(&rng.choose(&items).unwrap()).is_ok()));
    }

    #[bench]
    fn bench_std_binary_search_10k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).collect();
        let mut rng = thread_rng();
        b.iter(|| assert!(&items.binary_search(&rng.choose(&items).unwrap()).is_ok()));
    }

    #[test]
    fn test_linear_search() {
        assert_eq!(linear_search(&vec![9, 8, 7, 6], &9), Some(0));
        assert_eq!(linear_search(&vec![6, 8, 7, 9], &7), Some(2));
        assert_eq!(linear_search(&vec![2, 1, 1, 1, 1], &1), Some(1));
    }

    #[test]
    fn test_jump_search() {
        let mut v = vec![9, 8, 7, 6];
        v.sort();
        assert_eq!(jump_search(&v, &9, 2), Some(3));
        assert_eq!(jump_search(&v, &7, 2), Some(1));
        let mut v = vec![2, 1, 1, 1, 1];
        v.sort();
        assert_eq!(jump_search(&v, &1, 2), Some(2));
    }

    #[test]
    fn test_binary_search() {
        let mut v = vec![9, 8, 7, 6];
        v.sort();
        assert_eq!(binary_search(&v, &9), Some(3));
        assert_eq!(binary_search(&v, &7), Some(1));
        let mut v = vec![2, 1, 1, 1, 1];
        v.sort();
        assert_eq!(binary_search(&v, &1), Some(2));
    }

}
