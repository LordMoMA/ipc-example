use ipc_example::shared_memory::SharedCounter;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting shared memory child process");

    // Open existing shared memory
    let counter = SharedCounter::open("my_counter");

    // Child process increments counter
    for _ in 0..5 {
        thread::sleep(Duration::from_millis(750));
        let value = counter.increment();
        println!("Child incremented counter to: {}", value);
    }
}
