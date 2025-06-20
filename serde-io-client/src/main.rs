use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Ping,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Pong,
    Error(String),
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6380")?;
    let mut reader = BufReader::new(stream.try_clone()?);

    let cmd = Command::Ping;
    let serialized = serde_json::to_string(&cmd).unwrap();

    stream.write_all(serialized.as_bytes())?;
    stream.write_all(b"\n")?;

    let mut response_line = String::new();
    reader.read_line(&mut response_line)?;

    let response: Response = serde_json::from_str(response_line.trim()).unwrap();
    println!("Revieved: {:?}", response);

    Ok(())
}
