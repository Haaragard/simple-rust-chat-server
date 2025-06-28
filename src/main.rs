use std::{io::{Write}, net::{TcpListener, TcpStream}};

pub mod http;

fn main() {
    println!("Started: Best TCP Server in the world.\n\n");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on port 8080\n");

    for request in listener.incoming() {
        match request {
            Ok(mut stream) => {
                handle_request(&mut stream);
            },
            Err(e) => {
                println!("An error ocurred into the request: {}", e.to_string())
            },
        }
    }
}

pub fn handle_request(stream: &mut TcpStream) {
    let request_data = http::get_stream_data(stream).unwrap();

    let request = http::Request::new(request_data);
dbg!(&request);

    let response = response();
    stream.write_all(response.as_bytes()).unwrap();
}

fn response() -> String {
    return format!("HTTP/1.1 200 OK\r\n\r\n");
}
