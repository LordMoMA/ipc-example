use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Increment,
    GetValue,
    ValueResponse(u64),
}

pub fn send_message(stream: &mut UnixStream, message: &Message) -> std::io::Result<()> {
    let serialized = serde_json::to_string(message)?;
    let length = serialized.len() as u32;

    // Send message length first
    stream.write_all(&length.to_le_bytes())?;

    // Send the actual message
    stream.write_all(serialized.as_bytes())?;
    stream.flush()?;

    Ok(())
}

pub fn receive_message(stream: &mut UnixStream) -> std::io::Result<Message> {
    // Read message length
    let mut length_bytes = [0u8; 4];
    stream.read_exact(&mut length_bytes)?;
    let length = u32::from_le_bytes(length_bytes) as usize;

    // Read the actual message
    let mut buffer = vec![0u8; length];
    stream.read_exact(&mut buffer)?;

    let message = serde_json::from_slice(&buffer)?;
    Ok(message)
}
