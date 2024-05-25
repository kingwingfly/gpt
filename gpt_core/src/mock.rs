use std::{
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
    time::Duration,
};

use crate::{data::Chunk, error::Result};

#[derive(Default)]
pub struct Mock {}

impl Mock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&self, port: u16, close_idle: Duration) -> Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        let mut instant = std::time::Instant::now();
        loop {
            if let Ok((stream, _)) = listener.accept() {
                handle_stream(stream);
                instant = std::time::Instant::now();
            } else if instant.elapsed() > close_idle {
                break;
            }
        }
        Ok(())
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
