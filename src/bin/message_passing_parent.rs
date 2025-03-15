use ipc_example::message_passing::{receive_message, send_message, Message};
use std::os::unix::net::UnixListener;
use std::process::Command;

/*
* The socket is used for message passing IPC, but it is not used for shared memory IPC.
*
* Message Passing IPC (Uses Sockets)
*
  1. In message passing, processes communicate by sending messages to each other.
  2. A Unix domain socket acts like a virtual pipe where one process writes a message and the other reads it.
  3. Our code sets up a socket at /tmp/counter_socket, and both the parent and child process send/receive messages through it.
  4. This ensures strong isolation since processes do not directly access each other’s memory.
*/

fn main() {
    println!("Starting message passing parent process");

    // Create socket
    let socket_path = "/tmp/counter_socket";
    let _ = std::fs::remove_file(socket_path); // Remove if exists,  prevents conflicts from stale files, a Unix domain socket is represented as a file in the filesystem.
    let listener = UnixListener::bind(socket_path).expect("Failed to bind to socket");

    // Start child process
    let child = Command::new("./target/debug/message_passing_child") // allows the parent and child process to communicate via Unix domain sockets
        .spawn()
        .expect("Failed to start child process");

    // Accept connection from child
    let (mut stream, _) = listener.accept().expect("Failed to accept connection");

    // Parent process counter
    let mut counter = 0;

    // Handle messages from child
    loop {
        match receive_message(&mut stream) {
            Ok(Message::Increment) => {
                counter += 1;
                println!("Parent received increment request, counter: {}", counter);
                send_message(&mut stream, &Message::ValueResponse(counter))
                    .expect("Failed to send response");
            }
            Ok(Message::GetValue) => {
                println!("Parent received get value request, counter: {}", counter);
                send_message(&mut stream, &Message::ValueResponse(counter))
                    .expect("Failed to send response");
            }
            Err(_) => {
                println!("Child disconnected");
                break;
            }
            _ => {}
        }
    }

    // Cleanup
    std::fs::remove_file(socket_path).ok();
}
