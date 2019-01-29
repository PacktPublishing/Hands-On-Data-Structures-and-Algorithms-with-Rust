//!
//! This crate was created to support Hands on Algorithms and Data Structures With Rust!
//!
//! Chapter 1  
//!

#![feature(uniform_paths)]

struct MyStruct {
    a: u8,
    b: u8,
    c: u8,
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::mem;

    #[test]
    fn check_mem_size() {
        assert_eq!(mem::size_of::<MyStruct>(), 3 * mem::size_of::<u8>());
        assert_eq!(
            mem::size_of::<[MyStruct; 2]>(),
            3 * mem::size_of::<u8>() * 2
        );
    }
}
