use std::error::Error;
use std::net::ToSocketAddrs;
use stp::client::StpClient;

pub struct ChatClient {
    stp: StpClient,
}

impl ChatClient {
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> Result<ChatClient, Box<dyn Error>> {
        let stp = StpClient::connect(addr)?;
        Ok(ChatClient { stp })
    }

    pub fn fetch_messages(&mut self) -> Result<String, Box<dyn Error>> {
        self.stp.send_message("fetch")
    }

    pub fn add_message(&mut self, msg: String) -> Result<(), Box<dyn Error>> {
        let response = self.stp.send_message(&format!("append:{}", msg))?;
        if response != "ok" {
            println!("failed to append: {}", response);
        }

        Ok(())
    }
}