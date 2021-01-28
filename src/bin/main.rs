use servidor_rust::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
extern crate servidor_rust;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7373").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connections(stream);
    }

    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connections(stream);
        });
    }
}

fn web_response(mut stream: TcpStream, content: String, _status_line: String) {
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
    stream.write(response.as_bytes()).unwrap(); //Nos ayuda a leer la cadena bytes que estamos recibiendo.
    stream.flush().unwrap(); //Esperará e impedirá que el programa continúe hasta que se escriban todos los bytes en la conexión.
}

fn handle_connections(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let status_line = "HTTP/1.1 200 OK";
        let content = fs::read_to_string("hello.html").unwrap();
        web_response(stream, content, status_line.to_string())
    } else if buffer.starts_with(sleep) {
        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
        let content = fs::read_to_string("sleep.html").unwrap();
        thread::sleep(Duration::from_secs(2));
        web_response(stream, content, status_line.to_string())
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let content = fs::read_to_string("404.html").unwrap();
        web_response(stream, content, status_line.to_string())
    }
}
