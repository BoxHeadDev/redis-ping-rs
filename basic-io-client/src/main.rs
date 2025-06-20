use std::io::{Read, Result, Write};
use std::net::TcpStream;

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;

    stream.write_all(b"*1\r\n$4\r\nPING\r\n")?;

    let mut buffer = [0u8; 512];

    let n = stream.read(&mut buffer)?;

    let response = String::from_utf8_lossy(&buffer[..n]);

    println!("Received: {}", response);

    Ok(())
}
