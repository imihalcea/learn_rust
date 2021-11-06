use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Error as IoError, Read};
use std::net::TcpListener;

use log::{debug, error, info};

use crate::handler::Handler;
use crate::http::{ParseError, Request, Response};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self, handler: impl Handler) {
        let listener = TcpListener::bind(&self.address).unwrap();
        info!("Listening on: {}", self.address);
        loop{

            match self.accept_request(&listener, &handler){
                Ok(resp) => {
                    dbg!(resp);
                }
                Err(err) => {
                    error!("{}",err);
                    continue;
                }
            }
        }
    }

    fn accept_request<'buf>(&self, listener:&TcpListener, handler: &impl Handler) -> Result<Response, HandleError>{
        let mut buffer = [0;1024];

        let (mut stream, client_address) = listener.accept()?;
        debug!("Connected to: {}",client_address);

        let bytes_read = stream.read(&mut buffer)?;
        debug!("Received {}",String::from_utf8_lossy(&buffer[0..bytes_read]));


        let response = match Request::try_from(&buffer[0..bytes_read]) {
            Ok(req) => handler.handle_request(req),
            Err(error) => handler.handle_bad_request(error)
        };
        response.send(&mut stream)?;
        Ok(response)
    }
}

struct HandleError {
    message:String
}

impl From<IoError> for HandleError {
    fn from(error: std::io::Error) -> Self {
        Self{
            message:error.to_string()
        }
    }
}

impl From<ParseError> for HandleError {
    fn from(error: ParseError) -> Self {
        Self{
            message:String::from(error.message())
        }
    }
}


impl Debug for HandleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.message)
    }
}

impl<'err> Display for HandleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.message)
    }
}

impl Error for HandleError {

}
