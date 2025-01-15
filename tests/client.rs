

use std::io::{Read, Write};
use std::net::TcpStream;



pub struct Client {
    ip: String,
    port: u16,
}

impl Client {
    pub fn new(ip: &str, port: u16) -> Self {
        Client {
            ip: ip.to_string(),
            port,
        }
    }

    pub fn connect(&self) -> Result<TcpStream, std::io::Error> {
        TcpStream::connect(format!("{}:{}", self.ip, self.port))
    }

    pub fn send(&self, message: &str, stream: &mut TcpStream) -> Result<(), std::io::Error> {
       stream.write_all(message.as_bytes())
        // stream.flush() 
    }

    pub fn receive(&self, stream: &mut TcpStream) -> Result<String, std::io::Error> {
        let mut buffer: [u8; 512] = [0; 512]; // Adjust buffer size as needed
        let mut response: Vec<u8> = Vec::new();
    
        let bytes_read = stream.read(&mut buffer)?;

        response.extend_from_slice(&buffer[..bytes_read]);
    
        
        Ok(String::from_utf8_lossy(&response).to_string())
    }

}


