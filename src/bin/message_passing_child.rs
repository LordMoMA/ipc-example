use ipc_example::message_passing::{receive_message, send_message, Message};
use std::os::unix::net::UnixStream;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting message passing child process");

    // Connect to parent
    let mut stream =
        UnixStream::connect("/tmp/counter_socket").expect("Failed to connect to socket");

    // Send increment messages
    for i in 0..5 {
        println!("Child sending increment request {}", i + 1);
        send_message(&mut stream, &Message::Increment).expect("Failed to send message");

        // Get response
        if let Ok(Message::ValueResponse(value)) = receive_message(&mut stream) {
            println!("Child received counter value: {}", value);
        }

        thread::sleep(Duration::from_millis(500));
    }

    // Get final value
    send_message(&mut stream, &Message::GetValue).expect("Failed to send message");
    if let Ok(Message::ValueResponse(value)) = receive_message(&mut stream) {
        println!("Final counter value: {}", value);
    }
}
