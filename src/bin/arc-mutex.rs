use std::thread::Thread;
use std::sync::{Arc,Mutex};
use std::io;
fn main() {
    println!("Hello Arc and Mutex");
    let numbers = Arc::new(Mutex::new(vec![1u, 2u, 3u]));
    for i in range(0u, 3u) {
        let numbers_c = numbers.clone();
        Thread::spawn(move || {
            let mut numbers_l = numbers_c.lock();
            (*numbers_l)[i] += 1;
            println!("numbers[{}] is {}", i, (*numbers_l)[i]);
        }).detach();
    }

    let line: String = io::stdin().read_line().unwrap();
    println!("{}", line);
}
