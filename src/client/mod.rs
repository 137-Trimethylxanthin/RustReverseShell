use std::net::TcpStream;
use std::io::{self, Read, Write};
use crate::{CLIENT_IP, CLIENT_PORT};


pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn connect() -> io::Result<Self> {
        let stream = TcpStream::connect(format!("{}:{}", CLIENT_IP, CLIENT_PORT))?;
        Ok(Self { stream })
    }

    pub fn send(&mut self, message: &str) -> io::Result<()> {
        self.stream.write_all(message.as_bytes())
    }

    pub fn receive(&mut self) -> io::Result<String> {
        let mut buffer = [0; 512];
        let bytes_read = self.stream.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..bytes_read]).into_owned())
    }

    pub fn disconnect(&mut self) -> io::Result<()> {
        self.stream.shutdown(std::net::Shutdown::Both)
    }
}
