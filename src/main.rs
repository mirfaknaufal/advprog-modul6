use std::{
    fs,
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
}; 

use hello::ThreadPool;
 
fn main() { 
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); 
    let pool = ThreadPool::new(4);
 
    for stream in listener.incoming() { 
        let stream = stream.unwrap(); 
 
        pool.execute(|| {
            handle_connection(stream);
        });
    } 
} 
 
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = &http_request[0];

    let (status_line, contents) = if request_line.contains("GET / ") {
        ("HTTP/1.1 200 OK", fs::read_to_string("hello.html").unwrap())
    } else {
        ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("404.html").unwrap())
    };

    let (status_line, filename) = match &request_line[..] { 
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"), 
        "GET /sleep HTTP/1.1" => { 
        thread::sleep(Duration::from_secs(10)); 
        ("HTTP/1.1 200 OK", "hello.html") 
        } 
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"), 
        };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
