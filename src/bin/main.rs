use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

use web_server_rust::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) =
    if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let content = fs::read_to_string(filename).unwrap();

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, content.len(), content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}