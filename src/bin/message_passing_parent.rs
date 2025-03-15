use ipc_example::message_passing::{receive_message, send_message, Message};
use std::os::unix::net::UnixListener;
use std::process::Command;

fn main() {
    println!("Starting message passing parent process");

    // Create socket
    let socket_path = "/tmp/counter_socket";
    let _ = std::fs::remove_file(socket_path); // Remove if exists
    let listener = UnixListener::bind(socket_path).expect("Failed to bind to socket");

    // Start child process
    let child = Command::new("./target/debug/message_passing_child") 
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
