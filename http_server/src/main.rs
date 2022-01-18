use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

const ADDRESS: &str = "127.0.0.1:7878";
const HELLO_PAGE: &str = "pages/hello.html";
const NOT_FOUND_PAGE: &str = "pages/404.html";

fn main() {
  let listener = TcpListener::bind(ADDRESS).unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    println!("connection established!");
    handler_connection(stream);
  }
}

fn handler_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];

  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK", HELLO_PAGE)
  } else {
    ("HTTP/1.1 404 NOT FOUND", NOT_FOUND_PAGE)
  };

  let contents = fs::read_to_string(filename).unwrap();

  let response = format!(
    "{}\r\nContent-Length: {} Content-Type: text/html charset=UTF-8 \r\n\r\n{}",
    status_line,
    contents.len(),
    contents
  );

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();

  println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
