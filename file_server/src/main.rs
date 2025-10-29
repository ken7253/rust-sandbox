use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};

mod http;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8880")?;

    println!("[Address]: {}", listener.local_addr()?);

    for stream in listener.incoming() {
        handler(stream?);
    }

    return Ok(());
}

fn handler(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let response_body = "<!doctype html><html lang=\"en\"><head><title>Example Domain</title><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><body><h1>Example Domain</h1></body></html>";

    stream
        .write(format!("HTTP/1.1 200 OK\n\n{}", response_body).as_bytes())
        .unwrap();
}
