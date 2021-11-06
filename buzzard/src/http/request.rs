use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::http::{method::MethodError, method::Method, QueryString, Path};

#[derive(Debug)]
pub struct Request<'buf>{
    path:Path<'buf>,
    query_string:Option<QueryString<'buf>>,
    method:Method,
}

impl<'buf> Request<'buf>{
    pub fn path(&self)->&Path{
        &self.path
    }

    pub fn method(&self) -> &Method{
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString>{
        self.query_string.as_ref()
    }
}

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError{
    pub(crate) fn message(&self) -> &str{
        match self {
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidEncoding => "Invalid Encoding",
            ParseError::InvalidProtocol => "Invalid Protocol",
            ParseError::InvalidMethod => "Invalid Method"
        }
    }
}

impl From<std::str::Utf8Error> for ParseError{
    fn from(_: std::str::Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError{
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.message())
    }
}

impl Error for ParseError {

}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let req = std::str::from_utf8(buf)?;
        let (method, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (mut path, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method:Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?'){
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }
        Ok(Self{
            path:Path::from(path),
            query_string,
            method
        })
    }
}

fn get_next_word(req:&str) -> Option<(&str,&str)>{
    for (i,c) in req.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&req[..i], &req[i+1..]));
        }
    }
    None
}