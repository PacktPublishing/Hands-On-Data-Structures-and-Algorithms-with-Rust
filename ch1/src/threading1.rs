use std::thread; 

fn threading() { 
    let x = 10;
    let handle = thread::spawn(|| { 
        println!("Hello from a thread, the number is ", x);
    }); 
    handle.join().unwrap(); 
}

fn main() {
    threading();
}