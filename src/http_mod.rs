/// http模块
pub mod http {
    use std::net::{TcpListener, TcpStream};
    use std::io::{Read, Write};
    use std::{fs};
    use std::collections::HashMap;
    struct ThreadPool;

    impl ThreadPool {
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);
            ThreadPool{}
        }
        pub fn execute<F>(&self, _f: F)
            where F: FnOnce() + Send + 'static{}
    }

    // 监听tcp端口
    pub(crate) fn listen(asset: HashMap<&str, String>) {
        if asset.get("index")  != None {
            println!("{:?}", asset.get("index"));
        }
        let listener = TcpListener::bind("127.0.0.1:7777").unwrap();
        println!("server listener on http://127.0.0.1:7777");
        let pool = ThreadPool::new(4);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(|| {
                handle_connection(stream);
            });
        }
    }
    // 处理单个的stream
    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];
        let get = b"GET / HTTP/1.1\r\n";
        let _index: String = fs::read_to_string("/Users/guobin/vscode/play-rust/src/index.html").unwrap();
        let _not_found: String = fs::read_to_string("/Users/guobin/vscode/play-rust/src/404.html").unwrap();
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        if buffer.starts_with(get) {
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", _index);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", _not_found);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
