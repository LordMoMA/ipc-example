use ipc_example::shared_memory::SharedCounter;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting shared memory parent process");

    // Create shared memory
    let counter = SharedCounter::create("my_counter");

    // Start child process
    let child = Command::new("./target/debug/shared-memory-child")
        .spawn()
        .expect("Failed to start child process");

    // Parent process increments counter
    for _ in 0..5 {
        let value = counter.increment();
        println!("Parent incremented counter to: {}", value);
        thread::sleep(Duration::from_millis(500));
    }

    // Wait for child to finish
    child.wait_with_output().expect("Failed to wait for child");

    println!("Final counter value: {}", counter.get());
}
