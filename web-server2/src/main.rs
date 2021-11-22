use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;
use std::time::Duration;

static ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    let tcp_listener = TcpListener::bind(ADDRESS).unwrap();

    for stream in tcp_listener.incoming() {
        let mut stream = stream.unwrap();

        handle_request(&mut stream);
    }
}

fn handle_request(mut stream: &TcpStream) -> () {
    // is the stream mutable?
    // can we pass non-referenced mut object?
    // how was the buffer defined?

    let mut buffer = vec![0u8; 1024];
    stream.read(&mut buffer).unwrap();
    let buffer_str = from_utf8(&buffer[..]).unwrap();

    let request_http_line = buffer_str.lines().next().unwrap();

    println!("{:?}", request_http_line);

    let (status_line, filepath) = match request_http_line {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "public/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::new(3, 0));
            ("HTTP/1.1 404 NotFound", "public/404.html")
        }
        _ => ("HTTP/1.1 404 NotFound", "public/404.html"),
    };

    let body = fs::read_to_string(filepath).unwrap();
    let response = format!("{}\n\n{}", status_line, body);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
