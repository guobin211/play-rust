// 命名空间
use std::fs;

pub mod http {
    use std::net::{TcpListener, TcpStream};
    use std::io::{Read, Write};
    use std::fs;

    /// 监听tcp端口
    pub(crate) fn listen() {
        let listener = TcpListener::bind("127.0.0.1:7777").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");
            handle_connection(stream);
        }
    }
    /// 处理stream流
    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];
        // 读请求
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        let get = b"GET / HTTP/1.1\r\n";
        // GET请求
        if buffer.starts_with(get) {
            let _content = fs::read_to_string("/Users/guobin/idea/play-rust/src/index.html").unwrap();
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", _content);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            let _content = fs::read_to_string("/Users/guobin/idea/play-rust/src/404.html").unwrap();
            let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", _content);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

fn main() {
    let _index: String = fs::read_to_string("/Users/guobin/idea/play-rust/src/index.html").unwrap();
    let _not_found: String = fs::read_to_string("/Users/guobin/idea/play-rust/src/404.html").unwrap();
    println!("index:\n {}", _index);
    http::listen();
}

