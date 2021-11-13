use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // TODO: stress test this server against Deno server
    let buffer_str = String::from_utf8_lossy(&buffer);
    let http_line = buffer_str.lines().next().unwrap();
    println!("Request: {}", http_line);

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "public/hello.html")
    } else if buffer.starts_with(sleep) {
        // ref to continue from:
        // https://doc.rust-lang.org/book/ch20-02-multithreaded.html#improving-throughput-with-a-thread-pool
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "public/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "public/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
