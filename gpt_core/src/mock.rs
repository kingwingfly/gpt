use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener},
    time::Duration,
};

use crate::{chat::Chat, data::Chunk, error::Result};

pub(crate) struct Mock {}

impl Mock {
    pub(crate) fn new() -> Self {
        Mock {}
    }

    pub(crate) fn run(&self, port: u16, close_after: std::time::Duration) -> Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        println!("Listening on: {}", addr);
        let mut last_received = std::time::Instant::now();
        loop {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = vec![];
                if let Ok(len) = stream.read(&mut buf) {
                    let req: Chat = serde_json::from_slice(&buf[..len])?;
                    if !req.stream() {
                        unimplemented!("Non-streaming mode is not implemented.")
                    }
                    let resp = "This is response from mock server.".split_whitespace();
                    for word in resp {
                        let chunk = Chunk::new(word);
                        let chunk = gen_resp(&serde_json::to_string(&chunk)?);
                        stream.write_all(chunk.as_bytes())?;
                        std::thread::sleep(Duration::from_millis(100));
                    }
                    last_received = std::time::Instant::now();
                }
            } else {
                // Check if the timeout has been reached
                if last_received.elapsed() >= close_after {
                    println!("No data received. Quitting...");
                    break;
                }
                // Sleep for a short duration to prevent busy-waiting
                std::thread::sleep(Duration::from_millis(100));
            }
        }

        Ok(())
    }
}

fn gen_resp(msg: &str) -> String {
    let msg = format!("data: {}\n\n", msg);
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        msg.len(),
        msg
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_test() {
        let mock = Mock::new();
        mock.run(3000, Duration::from_secs(60)).unwrap();
    }
}
