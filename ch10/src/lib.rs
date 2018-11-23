#![feature(uniform_paths, test, bind_by_move_pattern_guards)]
use std::collections::BinaryHeap;
use std::mem;

pub fn linear_search<T: PartialOrd + Clone>(haystack: &[T], needle: &T) -> Option<usize> {
    haystack.iter().pos()
}


#[cfg(test)]
mod tests {
    extern crate test;
    use crate::*;
    use rand::{thread_rng, Rng};
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::iter;
    use test::Bencher;

    fn random_number_vec(size: usize) -> Vec<i64> {
        let mut v: Vec<i64> = (0..size as i64).collect();
        let mut rng = thread_rng();
        rng.shuffle(&mut v);
        v
    }
/*
    thread_local!(static _5K_DATA: RefCell<Vec<i64>> = RefCell::new(vec![1, 10, 2,9,3,8,4,7,5,6]));
    thread_local!(static _1K_DATA: RefCell<Vec<i64>> = RefCell::new(vec![1,2,3,4,5,6,7,8,9,10]));
*/
    thread_local!(static _5K_DATA: RefCell<Vec<i64>> = RefCell::new(random_number_vec(5_000)));

    thread_local!(static _1K_DATA: RefCell<Vec<i64>> = RefCell::new(random_number_vec(1_000)));

    thread_local!(static _10K_DATA: RefCell<Vec<i64>> = RefCell::new(random_number_vec(10_000)));

    #[bench]
    fn bench_bubble_sort_1k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).collect();
        b.iter(|| bubble_sort(&items));
    }

    #[bench]
    fn bench_bubble_sort_5k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).collect();
        b.iter(|| bubble_sort(&items));
    }

    #[bench]
    fn bench_bubble_sort_10k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).collect();
        b.iter(|| bubble_sort(&items));
    }

    #[bench]
    fn bench_shell_sort_1k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).collect();
        b.iter(|| shell_sort(&items));
    }

    #[bench]
    fn bench_shell_sort_5k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).collect();
        b.iter(|| shell_sort(&items));
    }

    #[bench]
    fn bench_shell_sort_10k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).collect();
        b.iter(|| shell_sort(&items));
    }

    #[bench]
    fn bench_merge_sort_1k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).collect();
        b.iter(|| merge_sort(&items));
    }

    #[bench]
    fn bench_merge_sort_5k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).collect();
        b.iter(|| merge_sort(&items));
    }

    #[bench]
    fn bench_merge_sort_10k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).collect();
        b.iter(|| merge_sort(&items));
    }

    #[bench]
    fn bench_heap_sort_1k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).collect();
        b.iter(|| heap_sort(&items));
    }

    #[bench]
    fn bench_heap_sort_5k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).collect();
        b.iter(|| heap_sort(&items));
    }

    #[bench]
    fn bench_heap_sort_10k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).collect();
        b.iter(|| heap_sort(&items));
    }

    #[bench]
    fn bench_quick_sort_1k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).collect();
        b.iter(|| quick_sort(&items));
    }

    #[bench]
    fn bench_quick_sort_5k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).collect();
        b.iter(|| quick_sort(&items));
    }

    #[bench]
    fn bench_quick_sort_10k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).collect();
        b.iter(|| quick_sort(&items));
    }

    #[bench]
    fn bench_stdlib_sort_1k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).collect();
        b.iter(|| items.clone().sort());
    }

    #[bench]
    fn bench_stdlib_sort_5k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).collect();
        b.iter(|| items.clone().sort());
    }

    #[bench]
    fn bench_stdlib_sort_10k_asc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).collect();
        b.iter(|| items.clone().sort());
    }

    #[bench]
    fn bench_bubble_sort_1k_random(b: &mut Bencher) {
        _1K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| bubble_sort(&items));
        });
    }

    #[bench]
    fn bench_bubble_sort_5k_random(b: &mut Bencher) {
        _5K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| bubble_sort(&items));
        });
    }

    #[bench]
    fn bench_bubble_sort_10k_random(b: &mut Bencher) {
        _10K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| bubble_sort(&items));
        });
    }

    #[bench]
    fn bench_shell_sort_1k_random(b: &mut Bencher) {
        _1K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| shell_sort(&items));
        });
    }

    #[bench]
    fn bench_shell_sort_5k_random(b: &mut Bencher) {
        _5K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| shell_sort(&items));
        });
    }

    #[bench]
    fn bench_shell_sort_10k_random(b: &mut Bencher) {
        _10K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| shell_sort(&items));
        });
    }

    #[bench]
    fn bench_merge_sort_1k_random(b: &mut Bencher) {
        _1K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| merge_sort(&items));
        });
    }

    #[bench]
    fn bench_merge_sort_5k_random(b: &mut Bencher) {
        _5K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| merge_sort(&items));
        });
    }

    #[bench]
    fn bench_merge_sort_10k_random(b: &mut Bencher) {
        _10K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| merge_sort(&items));
        });
    }

    #[bench]
    fn bench_heap_sort_1k_random(b: &mut Bencher) {
        _1K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| heap_sort(&items));
        });
    }

    #[bench]
    fn bench_heap_sort_5k_random(b: &mut Bencher) {
        _5K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| heap_sort(&items));
        });
    }

    #[bench]
    fn bench_heap_sort_10k_random(b: &mut Bencher) {
        _10K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| heap_sort(&items));
        });
    }

    #[bench]
    fn bench_quick_sort_1k_random(b: &mut Bencher) {
        _1K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| quick_sort(&items));
        });
    }

    #[bench]
    fn bench_quick_sort_5k_random(b: &mut Bencher) {
        _5K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| quick_sort(&items));
        });
    }

    #[bench]
    fn bench_quick_sort_10k_random(b: &mut Bencher) {
        _10K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| quick_sort(&items));
        });
    }

    #[bench]
    fn bench_stdlib_sort_1k_random(b: &mut Bencher) {
        _1K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| items.clone().sort());
        });
    }

    #[bench]
    fn bench_stdlib_sort_5k_random(b: &mut Bencher) {
        _5K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| items.clone().sort());
        });
    }

    #[bench]
    fn bench_stdlib_sort_10k_random(b: &mut Bencher) {
        _10K_DATA.with(|cell| {
            let items = cell.borrow();
            b.iter(|| items.clone().sort());
        });
    }

    #[bench]
    fn bench_bubble_sort_1k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).rev().collect();
        b.iter(|| bubble_sort(&items));
    }

    #[bench]
    fn bench_bubble_sort_5k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).rev().collect();
        b.iter(|| bubble_sort(&items));
    }

    #[bench]
    fn bench_bubble_sort_10k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).rev().collect();
        b.iter(|| bubble_sort(&items));
    }

    #[bench]
    fn bench_shell_sort_1k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).rev().collect();
        b.iter(|| shell_sort(&items));
    }

    #[bench]
    fn bench_shell_sort_5k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).rev().collect();
        b.iter(|| shell_sort(&items));
    }

    #[bench]
    fn bench_shell_sort_10k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).rev().collect();
        b.iter(|| shell_sort(&items));
    }

    #[bench]
    fn bench_merge_sort_1k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).rev().collect();
        b.iter(|| merge_sort(&items));
    }

    #[bench]
    fn bench_merge_sort_5k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).rev().collect();
        b.iter(|| merge_sort(&items));
    }

    #[bench]
    fn bench_merge_sort_10k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).rev().collect();
        b.iter(|| merge_sort(&items));
    }

    #[bench]
    fn bench_heap_sort_1k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).rev().collect();
        b.iter(|| heap_sort(&items));
    }

    #[bench]
    fn bench_heap_sort_5k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).rev().collect();
        b.iter(|| heap_sort(&items));
    }

    #[bench]
    fn bench_heap_sort_10k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).rev().collect();
        b.iter(|| heap_sort(&items));
    }

    #[bench]
    fn bench_quick_sort_1k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).rev().collect();
        b.iter(|| quick_sort(&items));
    }

    #[bench]
    fn bench_quick_sort_5k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).rev().collect();
        b.iter(|| quick_sort(&items));
    }

    #[bench]
    fn bench_quick_sort_10k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).rev().collect();
        b.iter(|| quick_sort(&items));
    }

    #[bench]
    fn bench_stdlib_sort_1k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..1_000).rev().collect();
        b.iter(|| items.clone().sort());
    }

    #[bench]
    fn bench_stdlib_sort_5k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..5_000).rev().collect();
        b.iter(|| items.clone().sort());
    }

    #[bench]
    fn bench_stdlib_sort_10k_desc(b: &mut Bencher) {
        let items: Vec<i32> = (0..10_000).rev().collect();
        b.iter(|| items.clone().sort());
    }

    #[test]
    fn test_bubble_sort() {
        assert_eq!(bubble_sort(&vec![9, 8, 7, 6]), vec![6, 7, 8, 9]);
        assert_eq!(bubble_sort(&vec![6, 8, 7, 9]), vec![6, 7, 8, 9]);
        assert_eq!(bubble_sort(&vec![2, 1, 1, 1, 1]), vec![1, 1, 1, 1, 2]);
    }

    #[test]
    fn test_merge_sort() {
        assert_eq!(merge_sort(&vec![9, 8, 7, 6]), vec![6, 7, 8, 9]);
        assert_eq!(merge_sort(&vec![6, 8, 7, 9]), vec![6, 7, 8, 9]);
        assert_eq!(merge_sort(&vec![2, 1, 1, 1, 1]), vec![1, 1, 1, 1, 2]);
    }

    #[test]
    fn test_shell_sort() {
        assert_eq!(shell_sort(&vec![9, 8, 7, 6]), vec![6, 7, 8, 9]);
        assert_eq!(shell_sort(&vec![6, 8, 7, 9]), vec![6, 7, 8, 9]);
        assert_eq!(shell_sort(&vec![2, 1, 1, 1, 1]), vec![1, 1, 1, 1, 2]);
    }

    #[test]
    fn test_heap_sort() {
        assert_eq!(heap_sort(&vec![9, 8, 7, 6]), vec![6, 7, 8, 9]);
        assert_eq!(heap_sort(&vec![6, 8, 7, 9]), vec![6, 7, 8, 9]);
        assert_eq!(heap_sort(&vec![2, 1, 1, 1, 1]), vec![1, 1, 1, 1, 2]);
    }

    #[test]
    fn test_quick_sort() {
        assert_eq!(quick_sort(&vec![9, 8, 7, 6]), vec![6, 7, 8, 9]);
        assert_eq!(quick_sort(&vec![8, 9, 7, 6]), vec![6, 7, 8, 9]);

        assert_eq!(quick_sort(&vec![6, 8, 7, 9]), vec![6, 7, 8, 9]);
        assert_eq!(quick_sort(&vec![2, 1, 1, 1, 1]), vec![1, 1, 1, 1, 2]);
    }

    #[test]
    fn test_stdlib_sort() {
        let mut v = vec![9, 8, 7, 6];
        v.sort();
        assert_eq!(v, vec![6, 7, 8, 9]);
    }
}
