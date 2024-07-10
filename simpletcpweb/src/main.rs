use std::fs::read_to_string;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to Read Buffer");

    let request = String::from_utf8_lossy(&buffer[..]);
    // println!("received request: {}", request);
    let status_line = "HTTP/1.1 200 OK\r\n";
    let content = read_to_string("./src/index.html")
        .unwrap()
        .replace("{say}", format!("{}", request).as_str());
    let response = format!("{status_line}\r\n{content}");

    stream
        .write_all(response.as_bytes())
        .expect("Failed to Write Buffer");
    stream.flush().expect("Failed to flush stream");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");
    println!("Server listerning on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(ex) => {
                eprintln!("Failed to establish connection: {}", ex);
            }
        }
    }
}
