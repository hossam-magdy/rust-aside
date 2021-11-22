use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    str::from_utf8,
    thread,
    time::Duration,
};

pub fn handle_request(mut stream: &TcpStream) -> () {
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
