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

    let raw_request = String::from_utf8_lossy(&buffer[..]).to_string();

    println!(
        "[Req]:
    {}",
        raw_request
    );

    let mut parser = http::Http::new();
    let request = parser.parse(&raw_request);

    println!("{:?}", request);

    let response_body = "<!doctype html><html lang=\"en\"><head><title>Example Domain</title><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><body><h1>Example Domain</h1></body></html>";

    stream
        .write(format!("HTTP/1.1 200 OK\n\n{}", response_body).as_bytes())
        .unwrap();
}
