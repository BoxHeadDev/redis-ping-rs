// Import necessary traits and types for I/O and networking
use std::io::{Read, Result, Write};
use std::net::TcpStream;

fn main() -> Result<()> {
    // Connect to the TCP server at 127.0.0.1 on port 6379
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;

    // Send a Redis-style PING command using the Redis Serialization Protocol (RESP)
    // *1     → 1 array element
    // $4     → bulk string of length 4
    // PING   → the actual command
    stream.write_all(b"*1\r\n$4\r\nPING\r\n")?;

    // Create a buffer to hold the server's response
    let mut buffer = [0u8; 512];

    // Read the server's response into the buffer
    let n = stream.read(&mut buffer)?;

    // Convert the received bytes into a UTF-8 string (lossy for invalid UTF-8)
    let response = String::from_utf8_lossy(&buffer[..n]);

    // Print the response to the console
    println!("Received: {}", response);

    Ok(())
}
