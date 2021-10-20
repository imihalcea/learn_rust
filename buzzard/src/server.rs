use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use log::{info, error, debug};
use std::io::{Read, Error as IoError};
use std::net::TcpListener;
use crate::http::{ParseError, Request};

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
            let mut buffer:[u8;1024] = [0;1024];
            match self.read_request(&listener, &mut buffer){
                Ok(req) => {
                    dbg!(&req);
                }
                Err(err) => {
                    error!("{}",err);
                    continue;
                }
            }
        }
    }

    fn read_request<'buf>(&self,listener:&TcpListener, buffer:&'buf mut [u8]) -> Result<Request<'buf>, ReadRequestError>{
        let (mut stream, client_address) = listener.accept()?;
        debug!("Connected to: {}",client_address);
        let bytes_read = stream.read(buffer)?;
        debug!("Received {}",String::from_utf8_lossy(&buffer[0..bytes_read]));
        let req = Request::try_from(&buffer[0..bytes_read])?;
        Ok(req)
    }
}

struct ReadRequestError{
    message:String
}

impl From<IoError> for ReadRequestError{
    fn from(error: std::io::Error) -> Self {
        Self{
            message:error.to_string()
        }
    }
}

impl From<ParseError> for ReadRequestError{
    fn from(error: ParseError) -> Self {
        Self{
            message:String::from(error.message())
        }
    }
}


impl Debug for ReadRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.message)
    }
}

impl<'err> Display for ReadRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.message)
    }
}

impl Error for ReadRequestError{

}
