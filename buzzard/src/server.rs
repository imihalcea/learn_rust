use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use log::{info, error, debug};
use std::io::{Read, Write, Error as IoError};
use std::net::TcpListener;
use crate::http::{ParseError, Request, Response, StatusCode};

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

            match self.handle_request(&listener){
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

    fn handle_request(&self, listener:&TcpListener) -> Result<(), ReadRequestError>{
        let mut buffer = [0;1024];

        let (mut stream, client_address) = listener.accept()?;
        debug!("Connected to: {}",client_address);

        let bytes_read = stream.read(&mut buffer)?;
        debug!("Received {}",String::from_utf8_lossy(&buffer[0..bytes_read]));

        let req = Request::try_from(&buffer[0..bytes_read])?;
        dbg!(&req);
        let response = Response::new(StatusCode::NotFound, Some("<h1>Hello Buzz!</h1>".to_string()));
        response.send(&mut stream);
        Ok(())
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
