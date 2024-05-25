use std::{
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::mpsc::{channel, Sender},
    time::Duration,
};

use crate::data::Chunk;

pub struct Mock {
    tx: Sender<()>,
}

impl Mock {
    pub fn new(port: u16, close_idle: Duration) -> Self {
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            println!("Mock server is running on {}", addr);
            let listener = TcpListener::bind(addr).unwrap();
            listener.set_nonblocking(true).unwrap();
            let mut instant = std::time::Instant::now();
            loop {
                if let Ok((stream, _)) = listener.accept() {
                    handle_stream(stream);
                    instant = std::time::Instant::now();
                } else if (instant.elapsed() > close_idle) | rx.try_recv().is_ok() {
                    break;
                }
            }
        });

        Self { tx }
    }

    pub fn close(&self) {
        self.tx.send(()).ok();
    }
}

fn handle_stream(mut stream: TcpStream) {
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
    for world in ["Response ", "from ", "mock ", "server."] {
        let chunk = Chunk::new(world);
        let chunk = gen_resp(serde_json::to_string(&chunk).unwrap());
        stream.write_all(chunk.as_bytes()).unwrap();
        stream.flush().unwrap();
        std::thread::sleep(Duration::from_millis(100));
    }
    let chunk = gen_resp("[DONE]".to_string());
    stream.write_all(chunk.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}

fn gen_resp(data: String) -> String {
    format!("data: {}\n\n", data)
}
