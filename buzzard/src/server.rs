use std::io::Read;
use log::{info, error, debug};
use std::net::TcpListener;
use crate::http::Request;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        info!("Listening on: {}", self.address);
        loop{
            match listener.accept() {
                Ok((mut stream, client_address)) => {
                    debug!("Connected to: {}",client_address);
                    let mut buffer = [0;1024];
                    match  stream.read(&mut buffer) {
                        Ok(len) => {
                            debug!("Received {}",String::from_utf8_lossy(&buffer[0..len]));
                        }
                        Err(err) => {
                            error!("{}",err);
                        }
                    };
                }
                Err(err) => {
                    error!("{}",err);
                    continue;
                }
            }
        }
    }
}

