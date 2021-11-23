// 36 minutes:
// - 12min single thread
// - 16min ThreadPool, Worker and channel
// - 8min Drop for ThreadPool

use web_server4::ThreadPool;

use std::{
    fs::read_to_string,
    io::{Read, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

static ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_stream(stream);
        });
    }
}

fn handle_stream(mut stream: std::net::TcpStream) -> () {
    // TODO: what exactly is this type?
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let buffer_str = String::from_utf8_lossy(&buffer[..]); // TODO what is happenning here?
    let first_line = buffer_str.lines().next().unwrap();
    println!("{}", first_line);

    let (status_line, filepath) = match first_line {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "public/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(3));
            ("HTTP/1.1 200 OK", "public/index.html")
        }
        _ => ("HTTP/1.1 404 Not Found", "public/404.html"),
    };

    let content = read_to_string(filepath).unwrap();
    let response = format!("{}\n\n{}", status_line, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
