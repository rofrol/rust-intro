use std::thread::Thread;
use std::io;

fn main() {
    for _ in range(0u, 10u) {
        Thread::spawn(move || {
            println!("Hello world");
        }).detach();
    }
    let line: String = io::stdin().read_line().unwrap();
    println!("{}", line);
}
