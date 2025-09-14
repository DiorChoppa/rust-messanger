use std::error::Error;
use std::net::{TcpStream, ToSocketAddrs};
use crate::{receive_string, send_string};

pub struct StpClient {
    stream: TcpStream,
}

impl StpClient {
    pub fn connect<Addr: ToSocketAddrs>(addr: Addr) -> Result<StpClient, Box<dyn Error>> {
        let stream = TcpStream::connect(addr)?;

        // todo: handshake

        Ok(StpClient {stream})
    }

    pub fn send_message(&mut self, s: &str) -> Result<String, Box<dyn Error>> {
        send_string(s, &mut self.stream)?;
        let response = receive_string(&mut self.stream)?;
        Ok(response)
    }
}