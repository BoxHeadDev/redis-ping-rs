use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => return,
            Ok(n) => n,
            Err(_) => return,
        };

        let request = &buffer[..bytes_read];
        let request_str = String::from_utf8_lossy(request);

        if request_str.contains("PING") {
            let _ = stream.write_all(b"+PONG\r\n");
        } else {
            let _ = stream.write_all(b"-ERR unknown command\r\n");
        }
    }
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    println!("Server listening on 6379...");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            std::thread::spawn(move || {
                handle_client(stream);
            });
        }
    }

    Ok(())
}
