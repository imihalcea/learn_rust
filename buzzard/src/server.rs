use log::{info, error};
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        info!("Listening on {}", self.address);
        loop{
            let result = listener.accept();
            match result {
                Ok((stream, client_address)) => {}
                Err(err) => {
                    error!("{}",err);
                    continue;
                }
            }
        }
    }
}

