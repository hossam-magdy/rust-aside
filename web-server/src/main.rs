use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Error binding to \"127.0.0.1:7878\" ... Maybe the port is already used");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.expect("Error in TcpStream incoming ...");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .expect("Error reading buffer (of size 1024) from stream ...");

    // TODO: stress test this server against Deno server
    let buffer_str = String::from_utf8_lossy(&buffer);
    let http_line = buffer_str
        .lines()
        .next()
        .expect("Error extracting lines from request stream ...");
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

    let contents = fs::read_to_string(filename).expect("Error reading file to string ...");
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream
        .write(response.as_bytes())
        .expect("Error writing bytes to stream ...");
    stream.flush().expect(
        "Error flushing the stream. Not sure all buffered content reached their destination.",
    )
}
