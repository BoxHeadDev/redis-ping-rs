// Import necessary traits and types for reading/writing and networking
use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};

// Function to handle a single client's connection
fn handle_client(mut stream: TcpStream) {
    // Create a buffer to read incoming data from the client
    let mut buffer = [0u8; 512];

    loop {
        // Read data into the buffer from the stream
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => return,  // Connection closed by the client
            Ok(n) => n,       // Read n bytes successfully
            Err(_) => return, // An error occurred; close connection
        };

        // Extract only the read portion of the buffer
        let request = &buffer[..bytes_read];
        // Convert the byte slice to a UTF-8 string (lossy in case of invalid UTF-8)
        let request_str = String::from_utf8_lossy(request);

        // Check if the client sent a "PING" command
        if request_str.contains("PING") {
            // Send back "+PONG\r\n" response (like Redis)
            let _ = stream.write_all(b"+PONG\r\n");
        } else {
            // Send back an error response for unknown commands
            let _ = stream.write_all(b"-ERR unknown command\r\n");
        }
    }
}

fn main() -> Result<()> {
    // Bind the TCP listener to localhost on port 6379 (like Redis)
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    println!("Server listening on 6379...");

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        // If the connection was successfully established
        if let Ok(stream) = stream {
            // Spawn a new thread to handle the client independently
            std::thread::spawn(move || {
                handle_client(stream);
            });
        }
    }

    Ok(())
}
