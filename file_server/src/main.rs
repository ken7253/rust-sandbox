use std::io::{Read, Result};
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

    http::HttpParser::new();
}
