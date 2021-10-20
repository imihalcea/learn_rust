use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub enum Method{
    GET,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

pub struct MethodError {
}


impl FromStr for Method{
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD"=> Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS"=> Ok(Self::OPTIONS),
            "TRACE"=> Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError{})
        }
    }
}



impl Debug for MethodError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Method Error")
    }
}

impl Display for MethodError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Method Error")
    }
}

impl Error for MethodError {}

