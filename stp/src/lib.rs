use std::{error::Error, io::Write, net::TcpStream};
use std::io::Read;

pub fn send_string(s: &str, sender: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let data = s.as_bytes();
    let len = data.len() as u32;
    sender.write_all(&len.to_le_bytes())?;
    sender.write_all(data)?;

    Ok(())
}
pub fn receive_string(receiver: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let mut len_bytes = [0_u8; 4];
    receiver.read_exact(&mut len_bytes)?;
    let len = u32::from_le_bytes(len_bytes);
    let mut data = vec![0_u8; len as usize];
    receiver.read_exact(&mut data)?;

    Ok(String::from_utf8(data)?)
}
