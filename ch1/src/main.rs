//! 
//! This crate was created to support Hands on Algorithms and Data Structures With Rust!
//! 
//! Chapter 1  
//!

#![feature(uniform_paths)]
use std::thread; 
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

mod door;

fn shared_state() {
    let v = Arc::new(Mutex::new(vec![]));
    let  handles = (0..10).map(|i| {
        let numbers = Arc::clone(&v);
        thread::spawn(move || {
            let mut vector = numbers.lock().unwrap();
            (*vector).push(i);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", *v.lock().unwrap());
}

fn threading() { 
    let x = 10;
    let handle = thread::spawn(move || { 
        println!("Hello from a thread, the number is {}", x);
    }); 
    handle.join().unwrap(); 
}

fn channels() {
    const N: i32 = 10;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();

    let handles = (0..N).map(|i| {
        let _tx = tx.clone();
        thread::spawn(move || { 
            // don't use the result
            let _ = _tx.send(i).unwrap(); 
        })
    });
    // close all threads
    for h in handles {
        h.join().unwrap();
    }
    
    // receive N times
    let numbers: Vec<i32> = (0..N).map(|_| rx.recv().unwrap()).collect();
    
    println!("{:?}", numbers);
}

#[derive(Debug, Clone)]
struct FileName {
    name: Rc<String>,
    ext: Rc<String>
}

fn ref_counter() {
    let name = Rc::new(String::from("main"));
    let ext = Rc::new(String::from("rs"));

    for _ in 0..3 {
       let f = FileName { name: name.clone(), ext: ext.clone() };
       println!("{:?}", f);
    }
}

fn main() {
    threading();
    shared_state();
    channels();
    ref_counter();
}