// Import traits and types from the `serde` crate for serialization and deserialization
use serde::{Deserialize, Serialize};

// Import necessary I/O and networking traits and types
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

// Define an enum `Command` that can be serialized/deserialized.
// Currently, it has a single variant `Ping`.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Ping,
}

// Define an enum `Response` for handling server responses.
// It supports `Pong` and `Error` with a string message.
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Pong,
    Error(String),
}

fn main() -> std::io::Result<()> {
    // Attempt to connect to the server at localhost on port 6380.
    let mut stream = TcpStream::connect("127.0.0.1:6380")?;

    // Create a buffered reader to read responses from the stream.
    let mut reader = BufReader::new(stream.try_clone()?);

    // Create a Ping command.
    let cmd = Command::Ping;

    // Serialize the command into a JSON string.
    let serialized = serde_json::to_string(&cmd).unwrap();

    // Send the serialized command to the server.
    stream.write_all(serialized.as_bytes())?;
    stream.write_all(b"\n")?; // Add a newline to signal end of message.

    // Read the response from the server into a string.
    let mut response_line = String::new();
    reader.read_line(&mut response_line)?;

    // Deserialize the response from JSON.
    let response: Response = serde_json::from_str(response_line.trim()).unwrap();

    // Print the received response.
    println!("Revieved: {:?}", response);

    Ok(())
}
