// Import serialization/deserialization traits from serde
use serde::{Deserialize, Serialize};

// Import standard I/O and networking modules
use std::io::{BufRead, BufReader, BufWriter, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// Define a `Command` enum for handling different types of incoming commands.
// Currently, it only includes a `Ping` variant.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Ping,
}

// Define a `Response` enum for server replies.
// It can be a `Pong` (successful response to `Ping`) or an `Error` with a message.
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Pong,
    Error(String),
}

// Function to handle communication with a single client.
fn handle_client(stream: TcpStream) {
    // Create a buffered reader using a cloned stream (for reading).
    let reader = BufReader::new(match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to clone stream: {}", e);
            return;
        }
    });

    // Create a buffered writer for sending data back to the client.
    let mut writer = BufWriter::new(stream);

    // Process each incoming line from the client
    for line in reader.lines() {
        // Handle any I/O errors when reading a line
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                break;
            }
        };

        // Try to parse the received line as a `Command`
        let cmd: serde_json::Result<Command> = serde_json::from_str(&line);

        // Based on the parsed command, create an appropriate `Response`
        let response = match cmd {
            Ok(Command::Ping) => Response::Pong,
            Err(e) => Response::Error(format!("Parse error: {}", e)),
        };

        // Serialize the response into a JSON string
        let serialized = match serde_json::to_string(&response) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to serialize response: {}", e);
                break;
            }
        };

        // Send the serialized response followed by a newline, and flush the writer
        if writer.write_all(serialized.as_bytes()).is_err()
            || writer.write_all(b"\n").is_err()
            || writer.flush().is_err()
        {
            eprintln!("Failed to send response to client.");
            break;
        }
    }
}

// Main function to start the server
fn main() -> Result<()> {
    // Bind a TCP listener to localhost on port 6380
    let listener = TcpListener::bind("127.0.0.1:6380")?;

    println!("Server listening on 6380...");

    // Loop to accept and handle incoming client connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle each client concurrently
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                // Log connection errors
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
