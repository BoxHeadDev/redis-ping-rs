use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, BufWriter, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Ping,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Pong,
    Error(String),
}

fn handle_client(stream: TcpStream) {
    let reader = BufReader::new(match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to clone stream: {}", e);
            return;
        }
    });
    let mut writer = BufWriter::new(stream);

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                break;
            }
        };

        let cmd: serde_json::Result<Command> = serde_json::from_str(&line);
        let response = match cmd {
            Ok(Command::Ping) => Response::Pong,
            Err(e) => Response::Error(format!("Parse error: {}", e)),
        };

        let serialized = match serde_json::to_string(&response) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to serialize response: {}", e);
                break;
            }
        };

        if writer.write_all(serialized.as_bytes()).is_err()
            || writer.write_all(b"\n").is_err()
            || writer.flush().is_err()
        {
            eprintln!("Failed to send response to client.");
            break;
        }
    }
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6380")?;

    println!("Server listening on 6380...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
