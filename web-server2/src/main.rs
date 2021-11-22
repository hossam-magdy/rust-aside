use std::{net::TcpListener, thread};

use web_server2::handle_request::handle_request;

static ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    let tcp_listener = TcpListener::bind(ADDRESS).unwrap();

    for stream in tcp_listener.incoming() {
        let mut stream = stream.unwrap();

        // do we "move" in thread::spawn?
        thread::spawn(move || {
            handle_request(&mut stream);
        });
    }
}