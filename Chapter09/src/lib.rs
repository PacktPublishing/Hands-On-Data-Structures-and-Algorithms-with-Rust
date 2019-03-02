#![feature(test, bind_by_move_pattern_guards)]
use std::collections::BinaryHeap;

pub fn bubble_sort<T: PartialOrd + Clone>(collection: &[T]) -> Vec<T> {
    let mut result: Vec<T> = collection.into();
    for _ in 0..result.len() {
        let mut swaps = 0;
        for i in 1..result.len() {
            if result[i - 1] > result[i] {
                result.swap(i - 1, i);
                swaps += 1;
            }
        }
        if swaps == 0 {
            break;
        }
    }
    result
}

pub fn shell_sort<T: PartialOrd + Clone>(collection: &[T]) -> Vec<T> {
    let n = collection.len();
    let mut gap = n / 2;
    let mut result: Vec<T> = collection.into();

    while gap > 0 {
        for i in gap..n {
            let temp = result[i].clone();

            let mut j = i;
            while j >= gap && result[j - gap] > temp {
                result[j] = result[j - gap].clone();
                j -= gap;
            }
            result[j] = temp;
        }
        gap /= 2;
    }
    result
}

pub fn heap_sort<T: PartialOrd + Clone + Ord>(collection: &[T]) -> Vec<T> {
    let mut heap = BinaryHeap::new();
    for c in collection {
        heap.push(c.clone());
    }
    heap.into_sorted_vec()
}

pub fn merge_sort<T: PartialOrd + Clone>(collection: &[T]) -> Vec<T> {
    if collection.len() > 1 {
        let (l, r) = collection.split_at(collection.len() / 2);
        let sorted_l = merge_sort(l);
        let sorted_r = merge_sort(r);
        let mut result: Vec<T> = collection.into();
        let (mut i, mut j) = (0, 0);
        let mut k = 0;
        while i < sorted_l.len() && j < sorted_r.len() {
            if sorted_l[i] <= sorted_r[j] {
                result[k] = sorted_l[i].clone();
                i += 1;
            } else {
                result[k] = sorted_r[j].clone();
                j += 1;
            }
            k += 1;
        }
        while i < sorted_l.len() {
            result[k] = sorted_l[i].clone();
            k += 1;
            i += 1;
        }

        while j < sorted_r.len() {
            result[k] = sorted_r[j].clone();
            k += 1;
            j += 1;
        }

        result
    } else {
        collection.to_vec()
    }
}

fn partition<T: PartialOrd + Clone>(
    collection: &mut [T],
    low: usize,
    high: usize,
) -> usize {
    let pivot = collection[high].clone();
    let (mut i, mut j) = (low as i64 - 1, high as i64 + 1);

    loop {
        'lower: loop {
            i += 1;
            if i > j || collection[i as usize] >= pivot {
                break 'lower;
            }
        }

        'upper: loop {
            j -= 1;
            if i > j || collection[j as usize] <= pivot {
                break 'upper;
            }
        }

        if i > j {
            return j as usize;
        }
        collection.swap(i as usize, j as usize);
    }
}

fn quick_sort_r<T: PartialOrd + Clone>(collection: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot = partition(collection, low, high);
        quick_sort_r(collection, low, pivot);
        quick_sort_r(collection, pivot + 1, high);
    }
}

pub fn quick_sort<T: PartialOrd + Clone>(collection: &[T]) -> Vec<T> {
    let mut result = collection.to_vec();
    quick_sort_r(&mut result, 0, collection.len() - 1);
    result
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
