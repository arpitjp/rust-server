use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, 
    fs, 
    thread,
    time::Duration,
};
use rust_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6900").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_path) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let html_content = fs::read_to_string(file_path).unwrap();
    let length = html_content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{html_content}");
    stream.write_all(response.as_bytes()).unwrap();
}
