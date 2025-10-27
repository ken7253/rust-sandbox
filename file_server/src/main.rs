use std::{
    io::{Error, Read},
    net::{TcpListener, TcpStream},
};

fn handler(stream: TcpStream) {
    print!("{:?}", stream);
}

fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8880")?;

    println!("Address:{:?}", listener.local_addr()?);

    for stream in listener.incoming() {
        handler(stream?);
    }

    return Ok(());
}
