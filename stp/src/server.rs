use std::error::Error;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use crate::{receive_string, send_string};

pub struct StpServer {
    tcp_listener: TcpListener,
}

pub struct StpConnection {
    stream: TcpStream,
}

impl StpServer {
    pub fn bind<Addr: ToSocketAddrs>(addr: Addr) -> Result<StpServer, Box<dyn Error>> {
        let tcp_listener = TcpListener::bind(addr)?;
        Ok(StpServer { tcp_listener})
    }

    pub fn accept(&mut self) -> Result<StpConnection, Box<dyn Error>> {
        let (stream, _) = self.tcp_listener.accept()?;

        // todo: handshake

        Ok(StpConnection { stream })
    }
}

impl StpConnection {
    pub fn process_request<F>(&mut self, handler: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(String) -> String
    {
        let request = receive_string(&mut self.stream)?;
        let reply = handler(request);
        send_string(&reply, &mut self.stream)?;
        Ok(())
    }
}