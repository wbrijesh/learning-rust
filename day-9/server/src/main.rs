use std::io::{Read, Write};
use std::net::TcpListener;
use std::str::FromStr;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for connection in listener.accept()? {
        let mut connection = connection;

        let mut buf = [0; 1024];
        connection.read(&mut buf)?;

        let request = String::from_utf8(buf.to_vec())?;
        println!("Received request: {}", request);

        let response = "Hello, world!";
        connection.write(response.as_bytes())?;
    }
}

